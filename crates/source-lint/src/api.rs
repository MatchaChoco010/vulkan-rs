use std::collections::BTreeSet;
use std::ops::Range;

use proc_macro2::Span;
use ra_ap_rustc_lexer::strip_shebang;
use syn::ext::IdentExt;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    AttrStyle, Attribute, Expr, Fields, ForeignItem, ImplItem, Item, Lit, Meta, Path, Token,
    TraitItem, Visibility,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildModule {
    pub name: String,
    pub inline_parents: Vec<String>,
    pub path_attribute: Option<String>,
    pub public: bool,
    pub test_only: bool,
}

#[derive(Debug, Default)]
pub struct PublicApi {
    pub documentable: BTreeSet<usize>,
    pub code: Vec<Range<usize>>,
    pub child_modules: Vec<ChildModule>,
}

pub fn public_api(
    source: &str,
    module_is_public: bool,
    module_is_test_only: bool,
) -> syn::Result<PublicApi> {
    let without_bom = source.strip_prefix('\u{feff}').unwrap_or(source);
    let body = &without_bom[strip_shebang(without_bom).unwrap_or(0)..];
    let file = syn::parse_str::<syn::File>(body)?;
    let mut walker = Walker {
        api: PublicApi::default(),
        shift: source.len() - body.len(),
        inline_parents: Vec::new(),
        inside_test_code: module_is_test_only || is_test_only(&file.attrs),
    };
    walker.document_if(module_is_public, &file.attrs);
    walker.items(&file.items, module_is_public);
    Ok(walker.api)
}

struct Walker {
    api: PublicApi,
    shift: usize,
    inline_parents: Vec<String>,
    inside_test_code: bool,
}

fn is_pub(visibility: &Visibility) -> bool {
    matches!(visibility, Visibility::Public(_))
}

fn is_named(path: &Path, name: &str) -> bool {
    path.get_ident().is_some_and(|ident| ident.unraw() == name)
}

fn is_test_only(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        is_named(attribute.path(), "test")
            || is_named(attribute.path(), "cfg")
                && attribute
                    .parse_args::<Meta>()
                    .is_ok_and(|predicate| requires_test(&predicate))
    })
}

fn requires_test(predicate: &Meta) -> bool {
    match predicate {
        Meta::Path(path) => is_named(path, "test"),
        Meta::List(list) => {
            let Ok(operands) =
                list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            else {
                return false;
            };
            if is_named(&list.path, "all") {
                operands.iter().any(requires_test)
            } else if is_named(&list.path, "any") {
                !operands.is_empty() && operands.iter().all(requires_test)
            } else {
                false
            }
        }
        Meta::NameValue(_) => false,
    }
}

fn attributes(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::ExternCrate(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::ForeignMod(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Static(item) => &item.attrs,
        Item::Struct(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::TraitAlias(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Use(item) => &item.attrs,
        _ => &[],
    }
}

fn string_value(attribute: &Attribute, name: &str) -> Option<String> {
    let Meta::NameValue(pair) = &attribute.meta else {
        return None;
    };
    if !pair.path.is_ident(name) {
        return None;
    }
    match &pair.value {
        Expr::Lit(literal) => match &literal.lit {
            Lit::Str(text) => Some(text.value()),
            _ => None,
        },
        _ => None,
    }
}

impl Walker {
    fn range(&self, span: Span) -> Range<usize> {
        let range = span.byte_range();
        range.start + self.shift..range.end + self.shift
    }

    fn document_if(&mut self, condition: bool, attributes: &[Attribute]) {
        if !condition || self.inside_test_code || is_test_only(attributes) {
            return;
        }
        for attribute in attributes {
            let start = self.range(attribute.pound_token.span).start;
            self.api.documentable.insert(start);
        }
    }

    fn code(&mut self, span: Span) {
        let range = self.range(span);
        self.api.code.push(range);
    }

    fn fields(&mut self, fields: &Fields, owner_is_public: bool, fields_inherit: bool) {
        for field in fields {
            self.document_if(
                owner_is_public && (fields_inherit || is_pub(&field.vis)),
                &field.attrs,
            );
        }
    }

    fn items(&mut self, items: &[Item], module_is_public: bool) {
        for item in items {
            self.item(item, module_is_public);
        }
    }

    fn item(&mut self, item: &Item, module_is_public: bool) {
        let enclosing = self.inside_test_code;
        self.inside_test_code = enclosing || is_test_only(attributes(item));
        self.item_contents(item, module_is_public);
        self.inside_test_code = enclosing;
    }

    fn item_contents(&mut self, item: &Item, module_is_public: bool) {
        match item {
            Item::Const(item) => {
                self.document_if(is_pub(&item.vis), &item.attrs);
                self.code(item.expr.span());
            }
            Item::Static(item) => {
                self.document_if(is_pub(&item.vis), &item.attrs);
                self.code(item.expr.span());
            }
            Item::Fn(item) => {
                self.document_if(is_pub(&item.vis), &item.attrs);
                self.code(item.block.brace_token.span.join());
            }
            Item::Struct(item) => {
                let public = is_pub(&item.vis);
                self.document_if(public, &item.attrs);
                self.fields(&item.fields, public, false);
            }
            Item::Union(item) => {
                let public = is_pub(&item.vis);
                self.document_if(public, &item.attrs);
                for field in &item.fields.named {
                    self.document_if(public && is_pub(&field.vis), &field.attrs);
                }
            }
            Item::Enum(item) => {
                let public = is_pub(&item.vis);
                self.document_if(public, &item.attrs);
                for variant in &item.variants {
                    self.document_if(public, &variant.attrs);
                    self.fields(
                        &variant.fields,
                        public && !is_test_only(&variant.attrs),
                        true,
                    );
                    if let Some((_, discriminant)) = &variant.discriminant {
                        self.code(discriminant.span());
                    }
                }
            }
            Item::Trait(item) => {
                let public = is_pub(&item.vis);
                self.document_if(public, &item.attrs);
                for member in &item.items {
                    match member {
                        TraitItem::Const(member) => {
                            self.document_if(public, &member.attrs);
                            if let Some((_, default)) = &member.default {
                                self.code(default.span());
                            }
                        }
                        TraitItem::Fn(member) => {
                            self.document_if(public, &member.attrs);
                            if let Some(block) = &member.default {
                                self.code(block.brace_token.span.join());
                            }
                        }
                        TraitItem::Type(member) => self.document_if(public, &member.attrs),
                        _ => {}
                    }
                }
            }
            Item::Impl(item) => {
                let inherent = item.trait_.is_none();
                for member in &item.items {
                    match member {
                        ImplItem::Const(member) => {
                            self.document_if(inherent && is_pub(&member.vis), &member.attrs);
                            self.code(member.expr.span());
                        }
                        ImplItem::Fn(member) => {
                            self.document_if(inherent && is_pub(&member.vis), &member.attrs);
                            self.code(member.block.brace_token.span.join());
                        }
                        ImplItem::Type(member) => {
                            self.document_if(inherent && is_pub(&member.vis), &member.attrs);
                        }
                        _ => {}
                    }
                }
            }
            Item::Mod(item) => {
                let public = module_is_public && is_pub(&item.vis);
                let (inner, outer): (Vec<_>, Vec<_>) = item
                    .attrs
                    .iter()
                    .cloned()
                    .partition(|attribute| matches!(attribute.style, AttrStyle::Inner(_)));
                self.document_if(is_pub(&item.vis), &outer);
                self.document_if(public, &inner);
                let name = item.ident.to_string();
                match &item.content {
                    Some((_, items)) => {
                        self.inline_parents.push(name);
                        self.items(items, public);
                        self.inline_parents.pop();
                    }
                    None => self.api.child_modules.push(ChildModule {
                        name,
                        inline_parents: self.inline_parents.clone(),
                        path_attribute: item
                            .attrs
                            .iter()
                            .find_map(|attribute| string_value(attribute, "path")),
                        public: public && !self.inside_test_code,
                        test_only: self.inside_test_code,
                    }),
                }
            }
            Item::ForeignMod(item) => {
                for member in &item.items {
                    match member {
                        ForeignItem::Fn(member) => {
                            self.document_if(is_pub(&member.vis), &member.attrs);
                        }
                        ForeignItem::Static(member) => {
                            self.document_if(is_pub(&member.vis), &member.attrs);
                        }
                        ForeignItem::Type(member) => {
                            self.document_if(is_pub(&member.vis), &member.attrs);
                        }
                        _ => {}
                    }
                }
            }
            Item::Type(item) => self.document_if(is_pub(&item.vis), &item.attrs),
            Item::TraitAlias(item) => self.document_if(is_pub(&item.vis), &item.attrs),
            Item::Use(item) => self.document_if(is_pub(&item.vis), &item.attrs),
            Item::ExternCrate(item) => self.document_if(is_pub(&item.vis), &item.attrs),
            Item::Macro(item) => {
                let exported = item
                    .attrs
                    .iter()
                    .any(|attribute| attribute.path().is_ident("macro_export"));
                self.document_if(exported, &item.attrs);
            }
            _ => {}
        }
    }
}

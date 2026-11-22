mod api;
mod comments;
mod versions;
mod workspace;

pub use api::ChildModule;
pub use comments::{Comment, CommentKind, find_comments};
pub use versions::{
    VersionMention, VersionReport, check_versions, find_version_mentions, markdown_files,
};
pub use workspace::{Report, check_crates};

pub const USER_FACING_CRATES: &[&str] = &["crates/vulkan", "crates/vulkan-alloc"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocComments {
    Forbidden,
    OnPublicApi { module_is_public: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    NoComments,
    NoDocCommentsOutsideUserFacingSources,
    NoDocCommentsInsideCode,
    NoDocCommentsOffThePublicApi,
}

impl Rule {
    pub fn describe(self) -> &'static str {
        match self {
            Rule::NoComments => "comments that are not doc comments are not allowed",
            Rule::NoDocCommentsOutsideUserFacingSources => {
                "doc comments are allowed only in the sources of the user-facing crates"
            }
            Rule::NoDocCommentsInsideCode => {
                "doc comments are not allowed inside function bodies and initializers"
            }
            Rule::NoDocCommentsOffThePublicApi => "doc comments are allowed only on the public API",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Violation {
    pub line: usize,
    pub kind: CommentKind,
    pub rule: Rule,
}

#[derive(Debug, Default)]
pub struct Checked {
    pub violations: Vec<Violation>,
    pub child_modules: Vec<ChildModule>,
}

pub fn check_source(source: &str, doc_comments: DocComments) -> syn::Result<Checked> {
    let comments = find_comments(source);
    let module_is_public = match doc_comments {
        DocComments::Forbidden => {
            return Ok(Checked {
                violations: comments
                    .into_iter()
                    .map(|comment| Violation {
                        line: comment.line,
                        kind: comment.kind,
                        rule: if comment.kind.is_doc() {
                            Rule::NoDocCommentsOutsideUserFacingSources
                        } else {
                            Rule::NoComments
                        },
                    })
                    .collect(),
                child_modules: Vec::new(),
            });
        }
        DocComments::OnPublicApi { module_is_public } => module_is_public,
    };
    let api = api::public_api(source, module_is_public)?;
    let violations = comments
        .into_iter()
        .filter_map(|comment| {
            let rule = if !comment.kind.is_doc() {
                Rule::NoComments
            } else if api.code.iter().any(|code| code.contains(&comment.offset)) {
                Rule::NoDocCommentsInsideCode
            } else if !api.documentable.contains(&comment.offset) {
                Rule::NoDocCommentsOffThePublicApi
            } else {
                return None;
            };
            Some(Violation {
                line: comment.line,
                kind: comment.kind,
                rule,
            })
        })
        .collect();
    Ok(Checked {
        violations,
        child_modules: api.child_modules,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_MODULE: DocComments = DocComments::OnPublicApi {
        module_is_public: true,
    };
    const PRIVATE_MODULE: DocComments = DocComments::OnPublicApi {
        module_is_public: false,
    };

    fn violations(source: &str, doc_comments: DocComments) -> Vec<(usize, Rule)> {
        check_source(source, doc_comments)
            .unwrap_or_else(|error| panic!("{source}: {error}"))
            .violations
            .into_iter()
            .map(|violation| (violation.line, violation.rule))
            .collect()
    }

    fn assert_allowed(source: &str) {
        assert_eq!(violations(source, PUBLIC_MODULE), [], "{source}");
    }

    fn assert_every_line_violates(source: &str, lines: &[usize], rule: Rule) {
        assert_eq!(
            violations(source, PUBLIC_MODULE),
            lines.iter().map(|line| (*line, rule)).collect::<Vec<_>>(),
            "{source}"
        );
    }

    #[test]
    fn where_doc_comments_are_forbidden_every_comment_form_violates() {
        let source = concat!(
            "//! crate\n",
            "/// item\n",
            "#[doc = \"attribute\"]\n",
            "pub fn f() {\n",
            "    // plain\n",
            "    /* block */\n",
            "}\n",
        );
        assert_eq!(
            violations(source, DocComments::Forbidden),
            [
                (1, Rule::NoDocCommentsOutsideUserFacingSources),
                (2, Rule::NoDocCommentsOutsideUserFacingSources),
                (3, Rule::NoDocCommentsOutsideUserFacingSources),
                (5, Rule::NoComments),
                (6, Rule::NoComments),
            ]
        );
    }

    #[test]
    fn where_doc_comments_are_forbidden_the_source_does_not_have_to_parse() {
        assert_eq!(
            violations("fn broken( { // why\n", DocComments::Forbidden),
            [(1, Rule::NoComments)]
        );
    }

    #[test]
    fn a_user_facing_source_that_does_not_parse_is_an_error() {
        assert!(check_source("pub fn broken( {\n", PUBLIC_MODULE).is_err());
    }

    #[test]
    fn plain_comments_violate_everywhere_in_user_facing_sources() {
        let source = concat!(
            "// above an item\n",
            "pub fn f() {\n",
            "    // SAFETY: in a body\n",
            "    let a = 1; /* inline */\n",
            "}\n",
            "pub struct S {\n",
            "    // on a field\n",
            "    pub a: u32,\n",
            "}\n",
        );
        assert_every_line_violates(source, &[1, 3, 4, 7], Rule::NoComments);
    }

    #[test]
    fn doc_comments_on_public_items_are_allowed() {
        assert_allowed(concat!(
            "//! The crate.\n",
            "#![doc = \"More about the crate.\"]\n",
            "/// A function.\n",
            "///\n",
            "/// # Safety\n",
            "///\n",
            "/// Read the manual.\n",
            "pub unsafe fn f() {}\n",
            "/** A constant. */\n",
            "pub const C: u32 = 1;\n",
            "/// A static.\n",
            "pub static S: u32 = 1;\n",
            "/// An alias.\n",
            "pub type T = u32;\n",
            "/// A re-export.\n",
            "pub use core::fmt::Debug;\n",
            "#[doc = \"An attribute.\"]\n",
            "#[cfg_attr(docsrs, doc = \"A conditional attribute.\")]\n",
            "#[derive(Clone)]\n",
            "pub struct Unit;\n",
        ));
    }

    #[test]
    fn doc_comments_on_the_members_of_public_types_are_allowed() {
        assert_allowed(concat!(
            "/// A struct.\n",
            "pub struct S {\n",
            "    /// A field.\n",
            "    pub a: u32,\n",
            "    b: u32,\n",
            "}\n",
            "/// A tuple struct.\n",
            "pub struct P(\n",
            "    /// Its field.\n",
            "    pub u32,\n",
            ");\n",
            "/// A union.\n",
            "pub union U {\n",
            "    /// A field.\n",
            "    pub a: u32,\n",
            "}\n",
            "/// An enum.\n",
            "pub enum E {\n",
            "    /// A variant.\n",
            "    A,\n",
            "    /// A variant with fields.\n",
            "    B {\n",
            "        /// A field of a variant.\n",
            "        a: u32,\n",
            "    },\n",
            "}\n",
            "/// A trait.\n",
            "pub trait Tr {\n",
            "    /// An associated constant.\n",
            "    const C: u32;\n",
            "    /// An associated type.\n",
            "    type T;\n",
            "    /// A method.\n",
            "    fn f(&self);\n",
            "}\n",
            "impl S {\n",
            "    /// An associated constant.\n",
            "    pub const C: u32 = 1;\n",
            "    /// A method.\n",
            "    pub fn f(&self) {}\n",
            "}\n",
        ));
    }

    #[test]
    fn doc_comments_inside_function_bodies_violate() {
        let source = concat!(
            "/// A function.\n",
            "pub fn f() {\n",
            "    /// On a statement.\n",
            "    let a = 1;\n",
            "    #[doc = \"On an expression.\"]\n",
            "    g(a);\n",
            "    /// On a local item, even a `pub` one.\n",
            "    pub struct Local;\n",
            "    let c = || {\n",
            "        //! Inside a closure.\n",
            "    };\n",
            "    m! {\n",
            "        /// Inside a macro invocation.\n",
            "        struct InMacro;\n",
            "    }\n",
            "}\n",
            "impl S {\n",
            "    pub fn f(&self) {\n",
            "        /// In a method body.\n",
            "        let a = 1;\n",
            "    }\n",
            "}\n",
            "impl T for S {\n",
            "    fn g(&self) {\n",
            "        /// In a trait method body.\n",
            "        let a = 1;\n",
            "    }\n",
            "}\n",
            "pub trait T {\n",
            "    fn g(&self) {\n",
            "        /// In a default method body.\n",
            "        let a = 1;\n",
            "    }\n",
            "}\n",
            "fn private() {\n",
            "    /// In a private function body.\n",
            "    let a = 1;\n",
            "}\n",
        );
        assert_every_line_violates(
            source,
            &[3, 5, 7, 10, 13, 19, 25, 31, 36],
            Rule::NoDocCommentsInsideCode,
        );
    }

    #[test]
    fn doc_comments_inside_initializers_violate() {
        let source = concat!(
            "/// A constant.\n",
            "pub const C: S = S {\n",
            "    /// On a field of the value.\n",
            "    a: 1,\n",
            "};\n",
            "/// A static.\n",
            "pub static T: u32 = {\n",
            "    /// On a statement.\n",
            "    let a = 1;\n",
            "    a\n",
            "};\n",
            "impl S {\n",
            "    /// An associated constant.\n",
            "    pub const D: u32 = {\n",
            "        /// On its value.\n",
            "        1\n",
            "    };\n",
            "}\n",
        );
        assert_every_line_violates(source, &[3, 8, 15], Rule::NoDocCommentsInsideCode);
    }

    #[test]
    fn doc_comments_on_items_that_are_not_public_violate() {
        let source = concat!(
            "/// Private function.\n",
            "fn f() {}\n",
            "/// Crate-visible struct.\n",
            "pub(crate) struct S {\n",
            "    /// Its public field.\n",
            "    pub a: u32,\n",
            "}\n",
            "/// Visible to the parent.\n",
            "pub(super) const C: u32 = 1;\n",
            "/// Private enum.\n",
            "enum E {\n",
            "    /// Its variant.\n",
            "    A,\n",
            "}\n",
            "/// Private trait.\n",
            "trait T {\n",
            "    /// Its method.\n",
            "    fn f(&self);\n",
            "}\n",
            "/// Private module.\n",
            "mod m {}\n",
            "/// Private import.\n",
            "use core::fmt::Debug;\n",
            "/// Crate-visible module.\n",
            "pub(crate) mod n {}\n",
        );
        assert_every_line_violates(
            source,
            &[1, 3, 5, 8, 10, 12, 15, 17, 20, 22, 24],
            Rule::NoDocCommentsOffThePublicApi,
        );
    }

    #[test]
    fn doc_comments_on_private_members_of_public_types_violate() {
        let source = concat!(
            "/// A struct.\n",
            "pub struct S {\n",
            "    /// A private field.\n",
            "    a: u32,\n",
            "    /// A crate-visible field.\n",
            "    pub(crate) b: u32,\n",
            "}\n",
            "impl S {\n",
            "    /// A private method.\n",
            "    fn f(&self) {}\n",
            "    /// A crate-visible constant.\n",
            "    pub(crate) const C: u32 = 1;\n",
            "}\n",
        );
        assert_every_line_violates(source, &[3, 5, 9, 11], Rule::NoDocCommentsOffThePublicApi);
    }

    #[test]
    fn doc_comments_on_trait_implementations_violate() {
        let source = concat!(
            "/// On the implementation.\n",
            "impl T for S {\n",
            "    /// On an implemented method.\n",
            "    fn f(&self) {}\n",
            "    /// On an implemented constant.\n",
            "    const C: u32 = 1;\n",
            "}\n",
            "/// On an inherent implementation.\n",
            "impl S {}\n",
        );
        assert_every_line_violates(source, &[1, 3, 5, 8], Rule::NoDocCommentsOffThePublicApi);
    }

    #[test]
    fn doc_comments_on_parameters_and_generics_violate() {
        let source = concat!(
            "/// A function.\n",
            "pub fn f<\n",
            "    /// On a generic parameter.\n",
            "    T,\n",
            ">(\n",
            "    /// On a parameter.\n",
            "    a: T,\n",
            ") {}\n",
        );
        assert_every_line_violates(source, &[3, 6], Rule::NoDocCommentsOffThePublicApi);
    }

    #[test]
    fn doc_comments_inside_item_macros_violate_unless_the_macro_is_exported() {
        let source = concat!(
            "/// A private macro.\n",
            "macro_rules! m {\n",
            "    () => {\n",
            "        /// Generated documentation.\n",
            "        pub struct S;\n",
            "    };\n",
            "}\n",
            "/// An exported macro.\n",
            "#[macro_export]\n",
            "macro_rules! n {\n",
            "    () => {};\n",
            "}\n",
        );
        assert_every_line_violates(source, &[1, 4], Rule::NoDocCommentsOffThePublicApi);
    }

    #[test]
    fn module_documentation_needs_a_public_module() {
        let source = "//! About this module.\n/// A function.\npub fn f() {}\n";
        assert_allowed(source);
        assert_eq!(
            violations(source, PRIVATE_MODULE),
            [(1, Rule::NoDocCommentsOffThePublicApi)]
        );
    }

    #[test]
    fn the_inner_documentation_of_a_module_needs_every_enclosing_module_to_be_public() {
        let source = concat!(
            "/// A public module.\n",
            "pub mod a {\n",
            "    //! More about it.\n",
            "    /// A nested public module.\n",
            "    pub mod b {}\n",
            "    mod c {\n",
            "        //! A private module.\n",
            "        /// A public module inside a private one, which can be re-exported.\n",
            "        pub mod d {\n",
            "            //! Its inner documentation.\n",
            "        }\n",
            "        /// A public item inside a private module.\n",
            "        pub fn f() {}\n",
            "    }\n",
            "}\n",
        );
        assert_every_line_violates(source, &[7, 10], Rule::NoDocCommentsOffThePublicApi);
        assert_eq!(
            violations(source, PRIVATE_MODULE)
                .into_iter()
                .map(|(line, _)| line)
                .collect::<Vec<_>>(),
            [3, 7, 10]
        );
    }

    #[test]
    fn child_modules_report_their_visibility_and_location() {
        let source = concat!(
            "pub mod a;\n",
            "mod b;\n",
            "pub mod c {\n",
            "    pub mod d;\n",
            "    #[path = \"elsewhere.rs\"]\n",
            "    pub(crate) mod e;\n",
            "}\n",
        );
        let child = |name: &str, inline_parents: &[&str], path: Option<&str>, public| ChildModule {
            name: name.to_string(),
            inline_parents: inline_parents.iter().map(|s| s.to_string()).collect(),
            path_attribute: path.map(str::to_string),
            public,
        };
        assert_eq!(
            check_source(source, PUBLIC_MODULE).unwrap().child_modules,
            [
                child("a", &[], None, true),
                child("b", &[], None, false),
                child("d", &["c"], None, true),
                child("e", &["c"], Some("elsewhere.rs"), false),
            ]
        );
        assert!(
            check_source(source, PRIVATE_MODULE)
                .unwrap()
                .child_modules
                .iter()
                .all(|module| !module.public)
        );
    }

    #[test]
    fn positions_survive_a_shebang_a_byte_order_mark_and_wide_characters() {
        let item = "/// Größe in µm, ≥ 3 × 𝛑.\npub fn f() {\n    /// → 𝛑.\n    let a = \"é\";\n}\n";
        for prefix in ["", "#!/usr/bin/env run\n", "\u{feff}"] {
            let source = format!("{prefix}{item}");
            let first_line = 1 + prefix.matches('\n').count();
            assert_eq!(
                violations(&source, PUBLIC_MODULE),
                [(first_line + 2, Rule::NoDocCommentsInsideCode)],
                "{source:?}"
            );
        }
    }

    #[test]
    fn hidden_and_alias_attributes_are_not_doc_comments() {
        assert_allowed("#[doc(hidden)]\nfn f() {\n    #[doc(alias = \"g\")]\n    struct S;\n}\n");
    }
}

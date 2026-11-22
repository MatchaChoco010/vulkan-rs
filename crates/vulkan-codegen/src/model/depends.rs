#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Depends {
    Version(u32, u32),
    Extension(String),
    All(Vec<Depends>),
    Any(Vec<Depends>),
}

impl Depends {
    pub(crate) fn parse(text: &str) -> Self {
        let tokens = Self::tokens(text);
        let mut position = 0;
        let parsed = Self::expression(&tokens, &mut position, text);
        assert_eq!(position, tokens.len(), "trailing input in `{text}`");
        parsed
    }

    fn tokens(text: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut name = String::new();
        for ch in text.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name.push(ch);
                continue;
            }
            if !name.is_empty() {
                tokens.push(std::mem::take(&mut name));
            }
            match ch {
                '(' | ')' | '+' | ',' => tokens.push(ch.to_string()),
                ch if ch.is_whitespace() => {}
                other => panic!("unexpected `{other}` in dependency `{text}`"),
            }
        }
        if !name.is_empty() {
            tokens.push(name);
        }
        tokens
    }

    fn expression(tokens: &[String], position: &mut usize, text: &str) -> Self {
        let mut left = Self::term(tokens, position, text);
        while let Some(op) = tokens.get(*position).filter(|t| *t == "+" || *t == ",") {
            *position += 1;
            let right = Self::term(tokens, position, text);
            left = match (op.as_str(), left) {
                ("+", Depends::All(mut all)) => {
                    all.push(right);
                    Depends::All(all)
                }
                (",", Depends::Any(mut any)) => {
                    any.push(right);
                    Depends::Any(any)
                }
                ("+", left) => Depends::All(vec![left, right]),
                (_, left) => Depends::Any(vec![left, right]),
            };
        }
        left
    }

    fn term(tokens: &[String], position: &mut usize, text: &str) -> Self {
        let token = tokens
            .get(*position)
            .unwrap_or_else(|| panic!("unexpected end of dependency `{text}`"));
        *position += 1;
        if token == "(" {
            let inner = Self::expression(tokens, position, text);
            assert_eq!(
                tokens.get(*position).map(String::as_str),
                Some(")"),
                "missing `)` in `{text}`"
            );
            *position += 1;
            return inner;
        }
        match parse_version(token) {
            Some((major, minor)) => Depends::Version(major, minor),
            None => {
                assert!(
                    token.starts_with("VK_"),
                    "unexpected name `{token}` in dependency `{text}`"
                );
                Depends::Extension(token.clone())
            }
        }
    }
}

pub(crate) fn parse_version(name: &str) -> Option<(u32, u32)> {
    let (major, minor) = name.strip_prefix("VK_VERSION_")?.split_once('_')?;
    Some((major.parse().ok()?, minor.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependency_expressions_associate_left_with_equal_precedence() {
        let ext = |name: &str| Depends::Extension(name.to_string());
        assert_eq!(Depends::parse("VK_KHR_surface"), ext("VK_KHR_surface"));
        assert_eq!(
            Depends::parse("VK_KHR_a+VK_KHR_b+VK_KHR_c"),
            Depends::All(vec![ext("VK_KHR_a"), ext("VK_KHR_b"), ext("VK_KHR_c")])
        );
        assert_eq!(
            Depends::parse("VK_KHR_a+VK_KHR_b,VK_VERSION_1_1"),
            Depends::Any(vec![
                Depends::All(vec![ext("VK_KHR_a"), ext("VK_KHR_b")]),
                Depends::Version(1, 1),
            ])
        );
        assert_eq!(
            Depends::parse("VK_KHR_a,VK_KHR_b+VK_KHR_c"),
            Depends::All(vec![
                Depends::Any(vec![ext("VK_KHR_a"), ext("VK_KHR_b")]),
                ext("VK_KHR_c"),
            ])
        );
        assert_eq!(
            Depends::parse(
                "((VK_KHR_extended_flags,VK_KHR_maintenance5)+(VK_KHR_buffer_device_address,VK_VERSION_1_2),VK_VERSION_1_4)"
            ),
            Depends::Any(vec![
                Depends::All(vec![
                    Depends::Any(vec![
                        ext("VK_KHR_extended_flags"),
                        ext("VK_KHR_maintenance5")
                    ]),
                    Depends::Any(vec![
                        ext("VK_KHR_buffer_device_address"),
                        Depends::Version(1, 2)
                    ]),
                ]),
                Depends::Version(1, 4),
            ])
        );
    }

    #[test]
    #[should_panic(expected = "missing `)`")]
    fn unbalanced_dependency_expressions_abort_generation() {
        Depends::parse("(VK_KHR_a+VK_KHR_b");
    }
}

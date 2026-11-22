use std::fs;
use std::path::{Path, PathBuf};

const TAG_PREFIX: &str = "vulkan-sdk-";
const SKIPPED_DIRECTORIES: &[&str] = &["third_party", "target"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VersionMention {
    pub line: usize,
    pub text: String,
    pub has_semver: bool,
}

#[derive(Debug, Default)]
pub struct VersionReport {
    pub scanned: Vec<PathBuf>,
    pub mentions: Vec<(PathBuf, VersionMention)>,
    pub violations: Vec<String>,
}

pub fn find_version_mentions(text: &str) -> Vec<VersionMention> {
    let is_version_char = |c: char| c.is_ascii_digit() || c == '.';
    let mut mentions = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let mut search_from = 0;
        while let Some(offset) = line[search_from..].find(TAG_PREFIX) {
            let start = search_from + offset;
            let number_start = start + TAG_PREFIX.len();
            let rest = &line[number_start..];
            let number = rest[..rest.find(|c| !is_version_char(c)).unwrap_or(rest.len())]
                .trim_end_matches('.');
            search_from = number_start + number.len();
            if !number.starts_with(|c: char| c.is_ascii_digit()) {
                continue;
            }
            let semver = line[..start].strip_suffix('+').map(|before| {
                before[before.trim_end_matches(is_version_char).len()..].trim_start_matches('.')
            });
            let (mention_start, has_semver) = match semver {
                Some(semver) if !semver.is_empty() => (start - 1 - semver.len(), true),
                _ => (start, false),
            };
            mentions.push(VersionMention {
                line: index + 1,
                text: line[mention_start..search_from].to_string(),
                has_semver,
            });
        }
    }
    mentions
}

pub fn markdown_files(workspace_root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_markdown_files(workspace_root, &mut files);
    files.sort();
    files
}

pub fn check_versions(workspace_root: &Path, version: &str) -> VersionReport {
    let (_, tag) = version
        .split_once('+')
        .unwrap_or_else(|| panic!("`{version}` has no `+<Vulkan-Headers SDK tag>` part"));
    let mut report = VersionReport::default();
    for file in markdown_files(workspace_root) {
        let name = file
            .strip_prefix(workspace_root)
            .unwrap_or(&file)
            .to_path_buf();
        let text =
            fs::read_to_string(&file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
        for mention in find_version_mentions(&text) {
            let expected = if mention.has_semver { version } else { tag };
            if mention.text != expected {
                report.violations.push(format!(
                    "{}:{}: `{}` does not match the workspace version `{version}` in Cargo.toml",
                    name.display(),
                    mention.line,
                    mention.text
                ));
            }
            report.mentions.push((name.clone(), mention));
        }
        report.scanned.push(name);
    }
    report
}

fn collect_markdown_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let entries =
        fs::read_dir(directory).unwrap_or_else(|e| panic!("read {}: {e}", directory.display()));
    for entry in entries {
        let path = entry.expect("directory entry").path();
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if path.is_dir() {
            if !name.starts_with('.') && !SKIPPED_DIRECTORIES.contains(&name) {
                collect_markdown_files(&path, files);
            }
        } else if path.extension().is_some_and(|extension| extension == "md") {
            files.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mentions(text: &str) -> Vec<(usize, String, bool)> {
        find_version_mentions(text)
            .into_iter()
            .map(|mention| (mention.line, mention.text, mention.has_semver))
            .collect()
    }

    #[test]
    fn finds_versions_and_sdk_tags() {
        let text = concat!(
            "for example\n",
            "`0.4.0+vulkan-sdk-1.4.363.0`.\n",
            "tag vulkan-sdk-1.4.363.0, then v1.2.3+vulkan-sdk-1.4.1.0.\n",
        );
        assert_eq!(
            mentions(text),
            [
                (2, "0.4.0+vulkan-sdk-1.4.363.0".to_string(), true),
                (3, "vulkan-sdk-1.4.363.0".to_string(), false),
                (3, "1.2.3+vulkan-sdk-1.4.1.0".to_string(), true),
            ]
        );
    }

    #[test]
    fn tag_patterns_are_not_versions() {
        assert_eq!(
            mentions("the newest `vulkan-sdk-*` tag, \"<semver>+vulkan-sdk-<x>\""),
            []
        );
    }

    #[test]
    fn a_stale_version_is_a_violation() {
        let root =
            std::env::temp_dir().join(format!("source-lint-versions-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove the old workspace");
        }
        for (path, contents) in [
            ("README.md", "`0.2.0+vulkan-sdk-1.4.350.0`\n"),
            ("crates/a/README.md", "`0.4.0+vulkan-sdk-1.4.363.0`\n"),
            ("crates/b/README.md", "\nvulkan-sdk-1.4.350.0\n"),
            ("third_party/c/README.md", "0.1.0+vulkan-sdk-1.0.0.0\n"),
            ("target/d.md", "0.1.0+vulkan-sdk-1.0.0.0\n"),
        ] {
            let path = root.join(path);
            fs::create_dir_all(path.parent().expect("parent")).expect("create a directory");
            fs::write(path, contents).expect("write a file");
        }
        let report = check_versions(&root, "0.4.0+vulkan-sdk-1.4.363.0");
        fs::remove_dir_all(&root).expect("remove the workspace");
        assert_eq!(
            report.scanned,
            [
                PathBuf::from("README.md"),
                PathBuf::from("crates/a/README.md"),
                PathBuf::from("crates/b/README.md"),
            ]
        );
        assert_eq!(
            report.violations,
            [
                "README.md:1: `0.2.0+vulkan-sdk-1.4.350.0` does not match the workspace version `0.4.0+vulkan-sdk-1.4.363.0` in Cargo.toml",
                "crates/b/README.md:2: `vulkan-sdk-1.4.350.0` does not match the workspace version `0.4.0+vulkan-sdk-1.4.363.0` in Cargo.toml",
            ]
        );
    }
}

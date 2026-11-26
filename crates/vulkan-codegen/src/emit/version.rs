use super::{header, w};
use crate::docs::Doc;
use crate::model::{Cfg, Model};
use crate::naming::const_name;

pub(crate) const MACROS_OF_EVERY_HEADER: [&str; 3] = [
    "VK_HEADER_VERSION",
    "VK_HEADER_VERSION_COMPLETE",
    "VK_API_VERSION_1_0",
];

fn numbers<const N: usize>(text: &str, separator: char) -> [u32; N] {
    let numbers = text
        .split(separator)
        .map(|part| {
            part.parse()
                .unwrap_or_else(|_| panic!("`{text}` is not a version"))
        })
        .collect::<Vec<u32>>();
    numbers
        .try_into()
        .unwrap_or_else(|_| panic!("`{text}` does not have {N} version numbers"))
}

pub(crate) fn emit(model: &Model<'_>) -> String {
    let ir = model.ir;
    let [major, minor, patch] = numbers(&ir.header_version_complete, '.');
    let mut out = header(&[]);
    let define = |c_name: &str| {
        Doc::new()
            .text(model.summary(&Cfg::default(), c_name).unwrap_or_default())
            .text(format!("C macro {}.", model.reference(c_name)))
            .text(model.legacy_type(&Cfg::default(), c_name))
            .alias(c_name)
            .render()
    };
    let [
        header_version_doc,
        header_version_complete_doc,
        api_version_1_0_doc,
    ] = MACROS_OF_EVERY_HEADER.map(define);
    w!(
        out,
        r#"{header_version_doc}pub const HEADER_VERSION: u32 = {header_version};
{header_version_complete_doc}pub const HEADER_VERSION_COMPLETE: u32 = make_api_version(0, {major}, {minor}, {patch});
/// The tag of Khronos Vulkan-Headers that these bindings were generated from.
pub const HEADERS_TAG: &str = "{tag}";
/// The commit of Khronos Vulkan-Headers that these bindings were generated from.
pub const HEADERS_COMMIT: &str = "{commit}";
/// The commit of Khronos Vulkan-Docs, the source of the Vulkan Specification, that the
/// descriptions in this documentation were taken from.
pub const DOCS_COMMIT: &str = "{docs_commit}";
{api_version_1_0_doc}pub const API_VERSION_1_0: u32 = make_api_version(0, 1, 0, 0);"#,
        header_version = ir.header_version,
        tag = ir.headers_tag,
        commit = ir.headers_commit,
        docs_commit = model.docs.commit,
    );
    for version in &ir.versions {
        let digits = version
            .name_api
            .strip_prefix("VK_API_VERSION_")
            .unwrap_or_else(|| panic!("{} is not a VK_API_VERSION_ name", version.name_api));
        let [major, minor] = numbers(digits, '_');
        w!(
            out,
            "{}pub const {}: u32 = make_api_version(0, {major}, {minor}, 0);",
            define(&version.name_api),
            const_name(&version.name_api)
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::registry;
    use vulkan_registry::Version;

    #[test]
    fn versions_are_packed_with_make_api_version() {
        let mut ir = registry();
        ir.versions.push(Version {
            name_api: "VK_API_VERSION_1_3".to_string(),
            ..Default::default()
        });
        let out = emit(&Model::new(&ir));
        assert!(out.contains("pub const HEADER_VERSION: u32 = 1;"));
        assert!(
            out.contains("pub const HEADER_VERSION_COMPLETE: u32 = make_api_version(0, 1, 4, 1);")
        );
        assert!(out.contains("pub const HEADERS_TAG: &str = \"vulkan-sdk-1.4.1.0\";"));
        assert!(out.contains("pub const API_VERSION_1_3: u32 = make_api_version(0, 1, 3, 0);"));
    }
}

use super::variants::{associated_consts, enum_variants, name_lookup};
use super::{header, type_aliases, w};
use crate::model::Model;
use crate::naming::type_name;

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&[]);
    for e in &model.ir.enums {
        assert_eq!(e.bit_width, 32, "{} is not a 32-bit enum", e.name);
        let cfg = model.cfg(e.protect.as_deref());
        let name = type_name(&e.name);
        let variants = enum_variants(model, e);
        let provenance = e
            .video_std_header
            .as_deref()
            .map(|header| model.provenance(&cfg, None, &[], Some(header)))
            .unwrap_or_default();
        w!(
            out,
            r#"{doc}{cfg}#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct {name}(pub i32);

{cfg}impl {name} {{
    /// Wraps a value of the C enumeration. Values without a constant are allowed.
    pub const fn from_raw(value: i32) -> Self {{ Self(value) }}
    /// The value as the C API passes it.
    pub const fn as_raw(self) -> i32 {{ self.0 }}
    {consts}{name_lookup}/// Whether a constant of this type has this value.
    pub fn is_known(self) -> bool {{ self.name().is_some() }}
}}

{cfg}impl core::fmt::Debug for {name} {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        debug_enum(f, "{name}", self.name(), self.0)
    }}
}}
"#,
            doc = model
                .type_doc(&cfg, &e.name, "C enumeration", &provenance)
                .render(),
            consts = associated_consts(model, &e.name, &cfg, &variants),
            name_lookup = name_lookup(&variants),
        );
        out.push_str(&type_aliases(&cfg, &e.aliases, &name, ""));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::{doc_of, squash};
    use super::*;
    use crate::test_support::{registry, run_with_the_tool};
    use vulkan_registry::{Enum, EnumField, Registry};

    #[test]
    fn enum_aliases_become_constants_but_not_debug_names() {
        let mut ir = registry();
        ir.enums.push(Enum {
            name: "VkPresentModeKHR".to_string(),
            bit_width: 32,
            fields: vec![EnumField {
                name: "VK_PRESENT_MODE_FIFO_LATEST_READY_KHR".to_string(),
                aliases: vec!["VK_PRESENT_MODE_FIFO_LATEST_READY_EXT".to_string()],
                value: "1000361000".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        });
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub const FIFO_LATEST_READY: Self = Self(1000361000);"));
        assert!(out.contains("pub const FIFO_LATEST_READY_EXT: Self = Self(1000361000);"));
        assert!(out.contains(
            "pub const fn variants() -> &'static [(Self, &'static str)] { &[ (Self(1000361000), \"FIFO_LATEST_READY\"), ] }"
        ));
        assert!(out.contains(
            "const fn aliases() -> &'static [(Self, &'static str)] { &[ (Self(1000361000), \"FIFO_LATEST_READY_EXT\"), ] }"
        ));
    }

    #[test]
    #[should_panic(expected = "already has value")]
    fn conflicting_constant_names_abort_generation() {
        let mut ir = registry();
        ir.enums.push(Enum {
            name: "VkThingKHR".to_string(),
            bit_width: 32,
            fields: vec![
                EnumField {
                    name: "VK_THING_A".to_string(),
                    value: "0".to_string(),
                    ..Default::default()
                },
                EnumField {
                    name: "VK_THING_A_KHR".to_string(),
                    value: "1".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        emit(&Model::new(&ir));
    }

    fn dumped_video_enumerants(video: &[&str]) -> Option<Result<Vec<String>, String>> {
        dumped_video_enumerants_after(&[], video)
    }

    fn dumped_video_enumerants_after(
        first: &[&str],
        video: &[&str],
    ) -> Option<Result<Vec<String>, String>> {
        let steps = [
            "with tempfile.TemporaryDirectory() as directory:",
            "    video = pathlib.Path(directory) / 'video.xml'",
            "    video.write_text(sys.stdin.read())",
            "    loaded, generator = tool.load_registry(tool.REGISTRY / 'vk.xml', video)",
            "    loaded.apiGen()",
            "for enumeration in generator.vk.videoStd.enums.values():",
            "    for field in enumeration.fields:",
            "        print(enumeration.name, field.name, field.value, *sorted(field.aliases))",
        ];
        let steps = first.iter().chain(&steps).copied().collect::<Vec<_>>();
        let dumped = run_with_the_tool(&steps, &[], &video.join("\n"))?;
        Some(dumped.map(|lines| lines.lines().map(str::to_string).collect()))
    }

    const VIDEO_REGISTRY: &[&str] = &[
        "<registry>",
        "  <types>",
        "    <type name='StdVideoThingKind' category='enum'/>",
        "    <type name='StdVideoThingMode' category='enum'/>",
        "  </types>",
        "  <enums name='StdVideoThingKind' type='enum'>",
        "    <enum name='STD_VIDEO_THING_KIND_PLAIN' value='0'/>",
        "    <enum name='STD_VIDEO_THING_KIND_ODD' value='1'/>",
        "    <enum name='STD_VIDEO_THING_KIND_SIMPLE' alias='STD_VIDEO_THING_KIND_PLAIN'/>",
        "    <enum name='STD_VIDEO_THING_KIND_BASIC' alias='STD_VIDEO_THING_KIND_SIMPLE' deprecated='aliased'/>",
        "    <enum name='STD_VIDEO_THING_KIND_INVALID' value='0x7FFFFFFF'/>",
        "  </enums>",
        "  <enums name='StdVideoThingMode' type='enum'>",
        "    <enum name='STD_VIDEO_THING_MODE_SLOW' value='0'/>",
        "    <enum name='STD_VIDEO_THING_MODE_FAST' value='1'/>",
        "    <enum name='STD_VIDEO_THING_MODE_QUICK' alias='STD_VIDEO_THING_MODE_FAST'/>",
        "  </enums>",
        "  <extensions>",
        "    <extension name='vulkan_video_codec_thing' supported='vulkan'>",
        "      <require>",
        "        <enum name='VK_STD_VULKAN_VIDEO_CODEC_THING_SPEC_VERSION' value='1'/>",
        "        <type name='StdVideoThingKind'/>",
        "        <type name='StdVideoThingMode'/>",
        "      </require>",
        "    </extension>",
        "  </extensions>",
        "</registry>",
    ];

    #[test]
    fn the_dump_gives_an_enumerant_of_the_video_headers_the_names_that_are_aliases_of_it() {
        let Some(dumped) = dumped_video_enumerants(VIDEO_REGISTRY) else {
            return;
        };
        assert_eq!(
            dumped.unwrap_or_else(|error| panic!("{error}")),
            [
                "StdVideoThingKind STD_VIDEO_THING_KIND_PLAIN 0 STD_VIDEO_THING_KIND_BASIC STD_VIDEO_THING_KIND_SIMPLE",
                "StdVideoThingKind STD_VIDEO_THING_KIND_ODD 1",
                "StdVideoThingKind STD_VIDEO_THING_KIND_INVALID 2147483647",
                "StdVideoThingMode STD_VIDEO_THING_MODE_SLOW 0",
                "StdVideoThingMode STD_VIDEO_THING_MODE_FAST 1 STD_VIDEO_THING_MODE_QUICK",
            ]
        );
    }

    #[test]
    fn the_dump_stops_at_an_alias_of_the_video_headers_that_ends_at_no_enumerant() {
        let registry = VIDEO_REGISTRY.join("\n");
        let alias = "alias='STD_VIDEO_THING_KIND_PLAIN'";
        assert_eq!(registry.matches(alias).count(), 1);
        let registry = registry.replacen(alias, "alias='STD_VIDEO_THING_KIND_GONE'", 1);
        let Some(dumped) = dumped_video_enumerants(&[&registry]) else {
            return;
        };
        let error = dumped.expect_err("an alias of no enumerant is dumped");
        assert!(
            error.trim_end().ends_with(
                "RuntimeError: STD_VIDEO_THING_KIND_SIMPLE of the Vulkan Video headers is an alias that ends at STD_VIDEO_THING_KIND_GONE, and the headers have no such enumerant"
            ),
            "{error}"
        );
    }

    #[test]
    fn the_dump_stops_when_the_generator_of_khronos_gives_the_aliases_itself() {
        let khronos_does_the_step = [
            "khronos = tool.khronos_scripts()[0]._VideoStdGenerator.__mro__[1]",
            "end_file = khronos.endFile",
            "def end_file_with_the_aliases(self):",
            "    for name, alias in self.enumFieldAliasMap.items():",
            "        enumerant = self.dealias(alias, self.enumFieldAliasMap)",
            "        self.enumFieldMap[enumerant].aliases.append(name)",
            "    end_file(self)",
            "khronos.endFile = end_file_with_the_aliases",
        ];
        let Some(dumped) = dumped_video_enumerants_after(&khronos_does_the_step, VIDEO_REGISTRY)
        else {
            return;
        };
        let error = dumped.expect_err("an alias is dumped twice");
        assert!(
            error.trim_end().ends_with(
                "RuntimeError: Khronos' generator of the Vulkan Video headers has given STD_VIDEO_THING_KIND_PLAIN its alias STD_VIDEO_THING_KIND_SIMPLE itself: the replacement of that generator in khronos_scripts has to go"
            ),
            "{error}"
        );
    }

    #[test]
    fn the_dump_stops_when_the_generator_put_in_place_of_the_one_of_khronos_does_not_run() {
        let message = "RuntimeError: Khronos' BaseGenerator has read the Vulkan Video headers without the generator that khronos_scripts puts in the place of its own, so their enumerants have no aliases: see how BaseGenerator.beginFile makes its generator of these headers now";
        for khronos_makes_another_generator in [
            [
                "scripts = tool.khronos_scripts()[0]",
                "scripts._VideoStdGenerator = scripts._VideoStdGenerator.__mro__[1]",
            ],
            [
                "scripts = tool.khronos_scripts()[0]",
                "scripts._VideoStdGenerator = type('Other', (scripts._VideoStdGenerator.__mro__[1],), {'ended': 1})",
            ],
        ] {
            let Some(dumped) =
                dumped_video_enumerants_after(&khronos_makes_another_generator, VIDEO_REGISTRY)
            else {
                return;
            };
            let error = dumped.expect_err("the enumerants are dumped without their aliases");
            assert!(error.trim_end().ends_with(message), "{error}");
        }
    }

    #[test]
    fn the_registry_data_has_the_alias_that_the_video_headers_have() {
        let enumerations = Registry::get().enums.iter();
        let of_the_video_headers = enumerations.filter(|e| e.video_std_header.is_some());
        let aliases = of_the_video_headers
            .flat_map(|e| e.fields.iter().map(move |field| (e, field)))
            .flat_map(|(e, field)| {
                let aliases = field.aliases.iter();
                aliases.map(move |alias| (e.name.as_str(), field.name.as_str(), alias.as_str()))
            })
            .collect::<Vec<_>>();
        assert_eq!(
            aliases,
            [(
                "StdVideoAV1ColorPrimaries",
                "STD_VIDEO_AV1_COLOR_PRIMARIES_UNSPECIFIED",
                "STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED"
            )]
        );
        let out = emit(&Model::new(Registry::get()));
        assert_eq!(
            doc_of(&out, "pub const BT_UNSPECIFIED: Self = Self(2);"),
            [
                "`STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED` is a legacy alias of [`STD_VIDEO_AV1_COLOR_PRIMARIES_UNSPECIFIED`](Self::UNSPECIFIED).",
                "#[doc(alias = \"STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED\")]",
            ]
        );
    }
}

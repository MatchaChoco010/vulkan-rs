mod expected;
mod output;
mod png;
mod programs;
mod trace;

use std::path::{Path, PathBuf};

use output::OutputImage;
use programs::Run;

macro_rules! scenarios {
    ($($name:ident),* $(,)?) => {
        const SCENARIOS: &[&str] = &[$(stringify!($name)),*];

        mod slow_render_compare {
            $(
                #[test]
                fn $name() {
                    crate::compare(stringify!($name), crate::expected::$name);
                }
            )*

            #[test]
            fn both_programs_have_the_scenarios_of_the_tests() {
                crate::both_programs_have(crate::SCENARIOS);
            }

            #[test]
            fn a_validation_message_fails_both_programs() {
                crate::a_validation_message_fails_both_programs();
            }
        }
    };
}

scenarios![
    cts_smoke_triangle,
    samples_hello_triangle_1_3,
    indexed_instanced,
    blend,
    depth_stencil,
    texture_mipmap,
    msaa_resolve,
    multiple_render_targets,
    compute_image,
    descriptor_kinds,
    secondary_queries,
    indirect_draws,
    tessellation_geometry,
    image_views,
    dynamic_rendering_msaa,
    descriptor_indexing,
    shader_objects,
    ray_query,
];

fn skip(reason: &str) {
    assert!(
        !programs::required(),
        "the render comparison is required but cannot run: {reason}"
    );
    eprintln!("skipping the render comparison: {reason}");
}

fn artifacts(scenario: &str) -> PathBuf {
    let directory = programs::workspace_root()
        .join("target/render-compare")
        .join(scenario);
    if directory.exists() {
        std::fs::remove_dir_all(&directory)
            .unwrap_or_else(|e| panic!("cannot remove {}: {e}", directory.display()));
    }
    std::fs::create_dir_all(&directory)
        .unwrap_or_else(|e| panic!("cannot create {}: {e}", directory.display()));
    directory
}

fn trace_wanted() -> Result<bool, String> {
    if std::env::var_os("VULKAN_RS_RENDER_COMPARE_TRACE").is_some_and(|value| value == "0") {
        return Ok(false);
    }
    trace::convert_available().map(|()| true)
}

fn write_pictures(directory: &Path, side: &str, images: &[OutputImage]) {
    for image in images.iter().filter(|image| image.is_picture()) {
        png::write_rgb(
            &directory.join(format!("{side}-{}.png", image.name)),
            image.width,
            image.height,
            &image.to_rgb(),
        );
    }
}

fn compare_images(directory: &Path, c: &[OutputImage], rust: &[OutputImage]) -> Vec<String> {
    let names = |images: &[OutputImage]| {
        images
            .iter()
            .map(|image| {
                format!(
                    "{} {} {}x{}",
                    image.name, image.format, image.width, image.height
                )
            })
            .collect::<Vec<_>>()
    };
    if names(c) != names(rust) {
        return vec![format!(
            "the programs read back different images: C {:?}, Rust {:?}",
            names(c),
            names(rust)
        )];
    }
    let mut failures = Vec::new();
    for (c, rust) in c.iter().zip(rust) {
        if c.data.len() != rust.data.len() {
            failures.push(format!(
                "{}: C read back {} bytes, Rust {}",
                c.name,
                c.data.len(),
                rust.data.len()
            ));
            continue;
        }
        let differing = c.texels_differing(rust);
        let count = differing.iter().filter(|&&differs| differs).count();
        if c.is_picture() {
            let rgb = c.to_rgb();
            let diff = differing
                .iter()
                .zip(rgb.chunks_exact(3))
                .flat_map(|(&differs, pixel)| {
                    if differs {
                        [255, 0, 0]
                    } else {
                        let gray = ((u16::from(pixel[0])
                            + u16::from(pixel[1])
                            + u16::from(pixel[2]))
                            / 12) as u8;
                        [gray, gray, gray]
                    }
                })
                .collect::<Vec<_>>();
            png::write_rgb(
                &directory.join(format!("diff-{}.png", c.name)),
                c.width,
                c.height,
                &diff,
            );
        }
        if count != 0 {
            let first = differing.iter().position(|&differs| differs).unwrap();
            failures.push(format!(
                "{}: {count} of {} texels differ; the first is texel {first} (x {}, y {})",
                c.name,
                differing.len(),
                first % c.width.max(1) as usize,
                first / c.width.max(1) as usize
            ));
        }
    }
    failures
}

fn compare_traces(directory: &Path, scenario: &str, c_program: &Path) -> Vec<String> {
    let mut traces = Vec::new();
    for (side, program) in [
        ("c", c_program.to_path_buf()),
        ("rust", programs::rust_program()),
    ] {
        let capture = directory.join(format!("{side}.gfxr"));
        let output = directory.join(format!("{side}-traced.out"));
        if let Run::Unsupported(reason) = programs::run(&program, scenario, &output, Some(&capture))
        {
            skip(&format!("cannot record the calls: {reason}"));
            return Vec::new();
        }
        let binaries = directory.join(format!("{side}-trace"));
        std::fs::create_dir_all(&binaries).expect("trace directory");
        let lines = trace::convert(&capture, &binaries);
        std::fs::remove_dir_all(&binaries).expect("remove the converted trace");
        std::fs::remove_file(&output).expect("remove the output of the traced run");
        std::fs::write(
            directory.join(format!("{side}.trace.jsonl")),
            lines.join("\n") + "\n",
        )
        .expect("write the normalized trace");
        traces.push(lines);
    }
    let (c, rust) = (&traces[0], &traces[1]);
    let mut failures = Vec::new();
    let calls = trace::calls(c);
    for wanted in [
        "vkCreateInstance",
        "vkCreateDevice",
        "vkQueueSubmit2",
        "vkDestroyInstance",
    ] {
        if !calls.iter().any(|call| call == wanted) {
            failures.push(format!("the trace of the C program has no {wanted}"));
        }
    }
    if !calls
        .iter()
        .any(|call| call.starts_with("vkCmdDraw") || call.starts_with("vkCmdDispatch"))
    {
        failures.push("the trace of the C program has no draw or dispatch".to_string());
    }
    std::fs::write(directory.join("calls.txt"), calls.join("\n") + "\n").expect("write calls");
    if let Some(difference) = trace::first_difference(c, rust) {
        std::fs::write(directory.join("trace-difference.txt"), &difference)
            .expect("write the trace difference");
        failures.push(difference);
    } else {
        eprintln!(
            "{scenario}: the C and Rust programs made the same {} Vulkan calls ({} trace entries)",
            calls.len(),
            c.len()
        );
    }
    failures
}

fn compare(scenario: &str, expect: fn(&mut expected::Expect, &[OutputImage])) {
    let c_program = match programs::c_program() {
        Ok(program) => program,
        Err(reason) => return skip(&reason),
    };
    let trace = match trace_wanted() {
        Ok(wanted) => wanted,
        Err(reason) => {
            skip(&format!("cannot record the calls: {reason}"));
            false
        }
    };
    let directory = artifacts(scenario);
    let c_output = directory.join("c.out");
    let rust_output = directory.join("rust.out");
    if let Run::Unsupported(reason) = programs::run(&c_program, scenario, &c_output, None) {
        return skip(&reason);
    }
    if let Run::Unsupported(reason) =
        programs::run(&programs::rust_program(), scenario, &rust_output, None)
    {
        panic!("the C program ran {scenario}, but the Rust program reports: {reason}");
    }
    let c = output::parse(&c_output);
    let rust = output::parse(&rust_output);
    write_pictures(&directory, "c", &c);
    write_pictures(&directory, "rust", &rust);

    let mut failures = compare_images(&directory, &c, &rust);
    let mut expect_c = expected::Expect::default();
    expect(&mut expect_c, &c);
    failures.extend(
        expect_c
            .failures
            .into_iter()
            .map(|failure| format!("the C output is not the expected picture: {failure}")),
    );
    if trace {
        failures.extend(compare_traces(&directory, scenario, &c_program));
    }
    assert!(
        failures.is_empty(),
        "{scenario} (pictures in {}):\n{}",
        directory.display(),
        failures.join("\n")
    );
}

fn both_programs_have(scenarios: &[&str]) {
    let c_program = match programs::c_program() {
        Ok(program) => program,
        Err(reason) => return skip(&reason),
    };
    let wanted = scenarios
        .iter()
        .map(|name| name.to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        programs::list(&c_program),
        wanted,
        "the scenarios of the C program"
    );
    assert_eq!(
        programs::list(&programs::rust_program()),
        wanted,
        "the scenarios of the Rust program"
    );
}

fn a_validation_message_fails_both_programs() {
    if !programs::validation_wanted() {
        eprintln!("the comparison runs without the validation layer");
        return;
    }
    let c_program = match programs::c_program() {
        Ok(program) => program,
        Err(reason) => return skip(&reason),
    };
    let directory = artifacts(programs::INVALID_CALL);
    for (side, program) in [("c", c_program), ("rust", programs::rust_program())] {
        let output = directory.join(format!("{side}.out"));
        if let Run::Unsupported(reason) = programs::run_invalid_call(&program, &output) {
            return skip(&reason);
        }
    }
}

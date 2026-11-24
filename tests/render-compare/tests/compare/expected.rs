use crate::output::OutputImage;

#[derive(Default)]
pub struct Expect {
    pub failures: Vec<String>,
}

const MAX_REPORTED: usize = 8;

impl Expect {
    pub fn that(&mut self, holds: bool, message: impl FnOnce() -> String) {
        if !holds {
            self.failures.push(message());
        }
    }

    pub fn rgba(&mut self, image: &OutputImage, x: u32, y: u32, wanted: [u8; 4], tolerance: u8) {
        let found = image.rgba8(x, y);
        let close = found
            .iter()
            .zip(wanted)
            .all(|(&found, wanted)| found.abs_diff(wanted) <= tolerance);
        self.that(close, || {
            format!(
                "{} at ({x}, {y}) is {found:?}, not {wanted:?} (tolerance {tolerance})",
                image.name
            )
        });
    }

    fn pixels(&mut self, what: &str, mismatches: Vec<String>) {
        if !mismatches.is_empty() {
            self.failures.push(format!(
                "{what}: {} pixels differ from the reference, among them {}",
                mismatches.len(),
                mismatches
                    .iter()
                    .take(MAX_REPORTED)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
}

pub fn image<'a>(images: &'a [OutputImage], name: &str) -> &'a OutputImage {
    images
        .iter()
        .find(|image| image.name == name)
        .unwrap_or_else(|| panic!("no output image named {name}"))
}

pub fn unorm8(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

pub fn srgb8(linear: f32) -> u8 {
    let linear = linear.clamp(0.0, 1.0);
    let encoded = if linear <= 0.003_130_8 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    };
    unorm8(encoded)
}

pub fn to_framebuffer(ndc: [f32; 2], width: u32, height: u32) -> [f32; 2] {
    [
        (ndc[0] + 1.0) * 0.5 * width as f32,
        (ndc[1] + 1.0) * 0.5 * height as f32,
    ]
}

pub struct Triangle {
    vertices: [[f32; 2]; 3],
    area: f32,
}

pub struct Coverage {
    pub barycentric: [f32; 3],
    pub inside: bool,
    pub distance_to_edge: f32,
}

impl Triangle {
    pub fn new(vertices: [[f32; 2]; 3]) -> Triangle {
        let area = edge(vertices[0], vertices[1], vertices[2]);
        assert!(area != 0.0, "a degenerate triangle");
        Triangle { vertices, area }
    }

    pub fn at_pixel(&self, x: u32, y: u32) -> Coverage {
        let point = [x as f32 + 0.5, y as f32 + 0.5];
        let [a, b, c] = self.vertices;
        let weights = [edge(b, c, point), edge(c, a, point), edge(a, b, point)];
        let barycentric = weights.map(|weight| weight / self.area);
        let lengths = [length(b, c), length(c, a), length(a, b)];
        let distance_to_edge = weights
            .iter()
            .zip(lengths)
            .map(|(weight, length)| (weight / self.area.signum()) / length)
            .fold(f32::INFINITY, |nearest, distance| {
                nearest.min(distance.abs())
            });
        Coverage {
            barycentric,
            inside: barycentric.iter().all(|&weight| weight >= 0.0),
            distance_to_edge,
        }
    }
}

fn edge(a: [f32; 2], b: [f32; 2], point: [f32; 2]) -> f32 {
    (b[0] - a[0]) * (point[1] - a[1]) - (b[1] - a[1]) * (point[0] - a[0])
}

fn length(a: [f32; 2], b: [f32; 2]) -> f32 {
    ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt()
}

fn close(found: [u8; 4], wanted: [u8; 4], tolerance: u8) -> bool {
    found
        .iter()
        .zip(wanted)
        .all(|(&found, wanted)| found.abs_diff(wanted) <= tolerance)
}

pub fn triangle_over_clear(
    expect: &mut Expect,
    image: &OutputImage,
    triangle: &Triangle,
    inside: impl Fn([f32; 3]) -> [u8; 4],
    clear: [u8; 4],
    tolerance: u8,
) {
    let mut mismatches = Vec::new();
    let mut covered = 0;
    for y in 0..image.height {
        for x in 0..image.width {
            let coverage = triangle.at_pixel(x, y);
            if coverage.distance_to_edge < 1.0 {
                continue;
            }
            let wanted = if coverage.inside {
                covered += 1;
                inside(coverage.barycentric)
            } else {
                clear
            };
            let found = image.rgba8(x, y);
            if !close(found, wanted, tolerance) {
                mismatches.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
            }
        }
    }
    expect.that(covered > 0, || {
        format!("{}: the triangle covers no pixel", image.name)
    });
    expect.pixels(&image.name, mismatches);
}

pub fn cts_smoke_triangle(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let vertices = [[-0.5, -0.5], [0.5, -0.5], [0.0, 0.5]]
        .map(|vertex| to_framebuffer(vertex, color.width, color.height));
    let clear = [unorm8(0.125), unorm8(0.25), unorm8(0.75), 255];
    triangle_over_clear(
        expect,
        color,
        &Triangle::new(vertices),
        |_| [255, 0, 255, 255],
        clear,
        0,
    );
}

pub fn samples_hello_triangle_1_3(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let vertices = [[0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]]
        .map(|vertex| to_framebuffer(vertex, color.width, color.height));
    let clear = [srgb8(0.01), srgb8(0.01), srgb8(0.033), 255];
    triangle_over_clear(
        expect,
        color,
        &Triangle::new(vertices),
        |[red, green, blue]| [srgb8(red), srgb8(green), srgb8(blue), 255],
        clear,
        2,
    );
}

pub fn indexed_instanced(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let clear = [24, 24, 32, 255];
    for instance in 0..16_u32 {
        let wanted = [
            (instance * 53 % 256) as u8,
            (instance * 97 % 256) as u8,
            ((instance * 29 + 64) % 256) as u8,
            255,
        ];
        let center_x = 32 + 64 * (instance % 4);
        let center_y = 32 + 64 * (instance / 4);
        for y in center_y - 20..center_y + 20 {
            for x in center_x - 20..center_x + 20 {
                expect.rgba(color, x, y, wanted, 0);
            }
        }
    }
    for line in 0..4 {
        for along in 0..color.width {
            expect.rgba(color, 64 * line, along, clear, 0);
            expect.rgba(color, along, 64 * line, clear, 0);
        }
    }
}

fn blend_equation(mode: u32, src: [f32; 4], dst: [f32; 4]) -> [f32; 4] {
    let constants = [0.25, 0.5, 0.75, 0.5];
    let alpha = |color: f32, alpha_value: f32, channel: usize| {
        if channel == 3 { alpha_value } else { color }
    };
    std::array::from_fn(|channel| {
        let (s, d) = (src[channel], dst[channel]);
        let value = match mode {
            0 => alpha(s * src[3], s, channel) + d * (1.0 - src[3]),
            1 => s + d,
            2 => s * alpha(d, dst[3], channel),
            3 => s * constants[channel] + d * (1.0 - constants[channel]),
            4 => s.min(d),
            5 => s.max(d),
            6 => d - s,
            _ => {
                if channel == 0 || channel == 3 {
                    s
                } else {
                    d
                }
            }
        };
        value.clamp(0.0, 1.0)
    })
}

fn quantize(color: [f32; 4]) -> [f32; 4] {
    color.map(|value| f32::from(unorm8(value)) / 255.0)
}

pub fn blend(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let unit = |bytes: [u8; 4]| bytes.map(|byte| f32::from(byte) / 255.0);
    let background = unit([64, 96, 128, 255]);
    let first = unit([200, 40, 40, 153]);
    let second = unit([30, 180, 220, 102]);
    for cell in 0..8_u32 {
        let (x, y) = (64 * (cell % 4), 128 * (cell / 4));
        let after_first = quantize(blend_equation(cell, first, background));
        let only_second = quantize(blend_equation(cell, second, background));
        let both = quantize(blend_equation(cell, second, after_first));
        for (dx, dy, wanted) in [
            (4, 4, background),
            (60, 124, background),
            (16, 24, after_first),
            (12, 80, after_first),
            (32, 64, both),
            (36, 44, both),
            (48, 104, only_second),
            (44, 116, only_second),
        ] {
            expect.rgba(color, x + dx, y + dy, wanted.map(unorm8), 1);
        }
    }
}

pub fn depth_stencil(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let depth = image(images, "depth");
    let stencil = image(images, "stencil");
    let clear = [16, 16, 16, 255];
    let red = [255, 0, 0, 255];
    let green = [0, 255, 0, 255];
    let blue = [0, 0, 255, 255];
    let green_depth = |x: u32| 0.5 + 0.4 * ((x as f32 + 0.5) / 128.0 - 1.0);
    for (x, y, wanted_color, wanted_depth, wanted_stencil) in [
        (16, 16, clear, 1.0, 0),
        (48, 48, red, 0.5, 1),
        (80, 100, green, green_depth(80), 1),
        (140, 100, red, 0.5, 1),
        (176, 100, green, green_depth(176), 0),
        (48, 176, blue, 0.5, 1),
        (80, 176, blue, green_depth(80), 1),
        (176, 176, green, green_depth(176), 0),
        (220, 176, clear, 1.0, 0),
    ] {
        expect.rgba(color, x, y, wanted_color, 0);
        let found = depth.d32f(x, y);
        expect.that((found - wanted_depth).abs() < 1e-3, || {
            format!("depth at ({x}, {y}) is {found}, not {wanted_depth}")
        });
        let found = stencil.s8(x, y);
        expect.that(found == wanted_stencil, || {
            format!("stencil at ({x}, {y}) is {found}, not {wanted_stencil}")
        });
    }
}

pub fn texture_mipmap(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let light = [240, 240, 240, 255];
    let dark = [16, 32, 128, 255];
    for y in 0..64 {
        for x in 0..64 {
            let wanted = if (x / 8 + y / 8) % 2 == 0 {
                light
            } else {
                dark
            };
            expect.rgba(color, x, y, wanted, 1);
        }
    }
    let average = [128, 136, 184, 255];
    for (quad_x, quad_y) in [(0, 64), (64, 64), (128, 64)] {
        for y in quad_y..quad_y + 64 {
            for x in quad_x..quad_x + 64 {
                expect.rgba(color, x, y, average, 2);
            }
        }
    }
    for y in 128..256 {
        for x in [0, 128, 255] {
            let pixel = color.rgba8(x, y);
            expect.that(pixel != [0, 0, 0, 255], || {
                format!("the floor does not cover ({x}, {y})")
            });
        }
    }
}

pub fn msaa_resolve(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let triangles = [
        ([[-0.9, -0.8], [0.7, -0.6], [-0.3, 0.9]], [255, 0, 0, 255]),
        ([[0.95, 0.1], [0.2, 0.95], [-0.1, -0.2]], [0, 255, 0, 255]),
        ([[-0.95, 0.7], [0.9, 0.85], [0.9, 0.8]], [0, 0, 255, 255]),
    ]
    .map(|(vertices, color_value)| {
        (
            Triangle::new(vertices.map(|vertex| to_framebuffer(vertex, 256, 256))),
            color_value,
        )
    });
    let levels = [0_u8, 64, 128, 191, 255];
    let mut interior = Vec::new();
    let mut not_an_average = Vec::new();
    let mut partly_covered = 0;
    for y in 0..color.height {
        for x in 0..color.width {
            let found = color.rgba8(x, y);
            let coverages = triangles
                .iter()
                .map(|(triangle, _)| triangle.at_pixel(x, y))
                .collect::<Vec<_>>();
            if coverages
                .iter()
                .all(|coverage| coverage.distance_to_edge >= 1.0)
            {
                let wanted = triangles
                    .iter()
                    .zip(&coverages)
                    .filter(|(_, coverage)| coverage.inside)
                    .map(|((_, color_value), _)| *color_value)
                    .next_back()
                    .unwrap_or([0, 0, 0, 255]);
                if found != wanted {
                    interior.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
                }
            } else {
                let averaged = found[3] == 255
                    && found[..3]
                        .iter()
                        .all(|&channel| levels.iter().any(|&level| channel.abs_diff(level) <= 1));
                if !averaged {
                    not_an_average.push(format!("({x}, {y}) {found:?}"));
                }
                if found[..3]
                    .iter()
                    .any(|&channel| (32..224).contains(&channel))
                {
                    partly_covered += 1;
                }
            }
        }
    }
    expect.pixels("pixels away from the edges", interior);
    expect.pixels(
        "edge pixels that are not an average of four samples",
        not_an_average,
    );
    expect.that(partly_covered > 300, || {
        format!("only {partly_covered} edge pixels are partly covered; was the image resolved from samples?")
    });
}

pub fn multiple_render_targets(expect: &mut Expect, images: &[OutputImage]) {
    let albedo = image(images, "albedo");
    let normal = image(images, "normal");
    let material = image(images, "material");
    let color = image(images, "color");
    let triangles = [
        ([[-0.9, -0.8], [0.7, -0.6], [-0.3, 0.9]], [1.0, 0.5, 0.0]),
        ([[0.95, 0.1], [0.2, 0.95], [-0.1, -0.2]], [0.0, 0.75, 0.25]),
        ([[-0.95, 0.7], [0.9, 0.85], [0.9, 0.8]], [0.25, 0.0, 1.0]),
    ]
    .map(|(vertices, rgb)| {
        (
            Triangle::new(vertices.map(|vertex| to_framebuffer(vertex, 256, 256))),
            rgb,
        )
    });
    let unit = |pixel: [u8; 4]| pixel.map(|channel| f32::from(channel) / 255.0);
    let mut gbuffer = Vec::new();
    let mut composite = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let (a, n, m) = (
                unit(albedo.rgba8(x, y)),
                unit(normal.rgba8(x, y)),
                unit(material.rgba8(x, y)),
            );
            let wanted = [
                unorm8(a[0] * n[0]),
                unorm8(a[1] * n[1] + m[1] * 0.25),
                unorm8(a[2].max(m[2] * 0.5)),
                255,
            ];
            if !close(color.rgba8(x, y), wanted, 1) {
                composite.push(format!(
                    "({x}, {y}) {:?} instead of {wanted:?}",
                    color.rgba8(x, y)
                ));
            }

            let coverages = triangles
                .iter()
                .map(|(triangle, _)| triangle.at_pixel(x, y))
                .collect::<Vec<_>>();
            if coverages
                .iter()
                .any(|coverage| coverage.distance_to_edge < 1.0)
            {
                continue;
            }
            let covering = triangles
                .iter()
                .zip(&coverages)
                .filter(|(_, coverage)| coverage.inside)
                .map(|((_, rgb), _)| *rgb)
                .next_back();
            let uv = [(x as f32 + 0.5) / 256.0, (y as f32 + 0.5) / 256.0];
            let (wanted_albedo, wanted_normal, wanted_material) = match covering {
                Some(rgb) => (
                    [unorm8(rgb[0]), unorm8(rgb[1]), unorm8(rgb[2]), 255],
                    [unorm8(uv[0]), unorm8(uv[1]), unorm8(0.5), 255],
                    [
                        unorm8(1.0 - rgb[0]),
                        unorm8(1.0 - rgb[1]),
                        unorm8(1.0 - rgb[2]),
                        255,
                    ],
                ),
                None => (
                    [0, 0, 0, 255],
                    [unorm8(0.5), unorm8(0.5), 255, 255],
                    [unorm8(0.25), unorm8(0.25), unorm8(0.25), 255],
                ),
            };
            for (target, wanted) in [
                (albedo, wanted_albedo),
                (normal, wanted_normal),
                (material, wanted_material),
            ] {
                let found = target.rgba8(x, y);
                if !close(found, wanted, 1) {
                    gbuffer.push(format!(
                        "{} ({x}, {y}) {found:?} instead of {wanted:?}",
                        target.name
                    ));
                }
            }
        }
    }
    expect.pixels("the attachments of the first subpass", gbuffer);
    expect.pixels("the attachment of the second subpass", composite);
}

pub fn compute_image(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let counters = image(images, "counters");
    let palette = [
        [255, 64, 32, 255],
        [32, 200, 64, 255],
        [16, 32, 240, 255],
        [240, 240, 16, 255],
    ];
    let mut counts = [0_u32; 4];
    let mut mismatches = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let cell = ((x / 32 + y / 32 + 1) % 4) as usize;
            counts[cell] += 1;
            let mut wanted = palette[cell];
            wanted[1] = ((x ^ y) & 255) as u8;
            let found = color.rgba8(x, y);
            if found != wanted {
                mismatches.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
            }
        }
    }
    expect.pixels("the storage image", mismatches);
    for (cell, &count) in counts.iter().enumerate() {
        let found = counters.word(cell);
        expect.that(found == count, || {
            format!("counter {cell} is {found}, not {count}")
        });
    }
    for group_y in 0..32 {
        for group_x in 0..32 {
            let found = counters.word(4 + group_y * 32 + group_x);
            let wanted = (group_x * 1000 + group_y) as u32;
            expect.that(found == wanted, || {
                format!("workgroup ({group_x}, {group_y}) wrote {found}, not {wanted}")
            });
        }
    }
}

pub fn descriptor_kinds(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let bands = [
        [200, 30, 30, 255],
        [30, 200, 30, 255],
        [30, 30, 200, 255],
        [200, 200, 30, 255],
        [30, 200, 200, 255],
        [200, 30, 200, 255],
        [120, 120, 120, 255],
        [90, 60, 30, 255],
    ];
    for (band, wanted) in bands.into_iter().enumerate() {
        let left = 32 * band as u32;
        for y in (0..color.height).step_by(16) {
            for x in [left, left + 16, left + 31] {
                expect.rgba(color, x, y, wanted, 0);
            }
        }
    }
}

pub fn secondary_queries(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let copied = image(images, "copied_queries");
    let host = image(images, "host_queries");
    for (query, samples) in [1024_u32, 0, 128].into_iter().enumerate() {
        let (low, high, found) = (
            copied.word(2 * query),
            copied.word(2 * query + 1),
            host.word(query),
        );
        expect.that(low == samples && high == 0 && found == samples, || {
            format!(
                "occlusion query {query}: copied {low} (high word {high}), read {found}, not {samples}"
            )
        });
    }
    for (x0, y0, x1, y1, wanted) in [
        (16, 16, 48, 48, [255, 0, 0, 255]),
        (64, 16, 80, 24, [0, 255, 0, 255]),
        (16, 64, 112, 112, [0, 0, 255, 255]),
        (128, 64, 240, 200, [255, 255, 0, 255]),
    ] {
        for (x, y) in [(x0, y0), (x1 - 1, y1 - 1), ((x0 + x1) / 2, (y0 + y1) / 2)] {
            expect.rgba(color, x, y, wanted, 0);
        }
        for (x, y) in [(x0 - 1, y0), (x1, y1 - 1)] {
            expect.rgba(color, x, y, [0, 0, 0, 255], 0);
        }
    }
}

pub fn indirect_draws(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let clear = [16, 16, 16, 255];
    let quad_a = (8, 8, 56, 56);
    let quad_b = (72, 40, 120, 120);
    let quad_c = (72, 8, 120, 24);
    let mut wanted = vec![Some(clear); 256 * 256];
    let mut fill = |quadrant: u32,
                    (x0, y0, x1, y1): (u32, u32, u32, u32),
                    value: Option<[u8; 4]>| {
        for y in y0..y1 {
            for x in x0..x1 {
                wanted[((128 * (quadrant / 2) + y) * 256 + 128 * (quadrant % 2) + x) as usize] =
                    value;
            }
        }
    };
    fill(0, quad_a, Some([220, 40, 40, 255]));
    fill(0, quad_b, Some([40, 220, 40, 255]));
    fill(1, quad_a, Some([240, 60, 30, 255]));
    fill(2, quad_a, Some([220, 220, 40, 255]));
    fill(2, quad_b, Some([220, 220, 40, 255]));
    fill(2, quad_c, Some([40, 220, 220, 255]));
    let line = Some([100, 50, 120, 255]);
    fill(3, (9, 16, 120, 17), line);
    fill(3, (16, 25, 17, 120), line);
    for end in [(8, 16), (120, 16), (16, 24), (16, 120)] {
        fill(3, (end.0, end.1, end.0 + 1, end.1 + 1), None);
    }
    for point in [(40, 40), (80, 40), (40, 80), (80, 80)] {
        fill(
            3,
            (point.0, point.1, point.0 + 1, point.1 + 1),
            Some([250, 250, 250, 255]),
        );
    }
    let mut mismatches = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let found = color.rgba8(x, y);
            if let Some(wanted) = wanted[(y * 256 + x) as usize]
                && found != wanted
            {
                mismatches.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
            }
        }
    }
    expect.pixels("the four quadrants", mismatches);
}

pub fn tessellation_geometry(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let clear = [16, 16, 16, 255];
    let palette = [[230, 60, 60, 255], [60, 230, 60, 255], [60, 60, 230, 255]];
    let mut mismatches = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let inside = (32..224).contains(&x) && (32..224).contains(&y);
            let quarter = ((x.max(32) - 32) / 96 + 2 * ((y.max(32) - 32) / 96)) as usize;
            let wanted = match palette.get(quarter) {
                Some(&wanted) if inside => wanted,
                _ => clear,
            };
            let found = color.rgba8(x, y);
            if found != wanted {
                mismatches.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
            }
        }
    }
    expect.pixels("the tessellated square", mismatches);
}

pub fn image_views(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let cells: [[u8; 3]; 15] = [
        [255, 0, 0],
        [0, 255, 0],
        [0, 0, 255],
        [255, 255, 0],
        [255, 0, 255],
        [0, 255, 255],
        [40, 80, 120],
        [40, 80, 120],
        [255, 255, 0],
        [200, 100, 50],
        [50, 100, 200],
        [125, 100, 125],
        [30, 200, 10],
        [255, 255, 255],
        [250, 10, 10],
    ];
    let mut mismatches = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let cell = (x / 64 + 4 * (y / 64)) as usize;
            let (wanted, tolerance) = match cells.get(cell) {
                Some(&[red, green, blue]) => ([red, green, blue, 255], u8::from(cell == 11)),
                None if x % 64 < 32 => ([255, 255, 255, 255], 0),
                None => ([0, 0, 0, 255], 0),
            };
            let found = color.rgba8(x, y);
            if !close(found, wanted, tolerance) {
                mismatches.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
            }
        }
    }
    expect.pixels("the sixteen cells", mismatches);
}

pub fn dynamic_rendering_msaa(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let depth = image(images, "depth");
    let resolved = image(images, "resolved");
    let within = |x: u32, y: u32, (x0, y0, x1, y1): (u32, u32, u32, u32)| {
        (x0..x1).contains(&x) && (y0..y1).contains(&y)
    };
    let red = (16, 16, 144, 144);
    let green = (80, 80, 208, 208);
    let blue = (112, 48, 240, 112);
    let cleared = (176, 16, 240, 48);
    let yellow = (16, 176, 80, 240);
    let mut colors = Vec::new();
    let mut depths = Vec::new();
    let mut resolves = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let (wanted_color, wanted_depth, tolerance) = if within(x, y, cleared) {
                ([255, 0, 255, 255], 0.25, 0)
            } else if within(x, y, red) {
                if within(x, y, blue) {
                    ([0, 0, 255, 255], 0.5, 0)
                } else {
                    ([255, 0, 0, 255], 0.5, 0)
                }
            } else if within(x, y, green) {
                ([0, 255, 0, 255], 0.75, 0)
            } else if within(x, y, yellow) {
                ([143, 143, 24, 255], 1.0, 1)
            } else {
                ([32, 32, 48, 255], 1.0, 0)
            };
            let found = color.rgba8(x, y);
            if !close(found, wanted_color, tolerance) {
                colors.push(format!("({x}, {y}) {found:?} instead of {wanted_color:?}"));
            }
            let found = depth.d32f(x, y);
            if (found - wanted_depth).abs() > 1e-4 {
                depths.push(format!("({x}, {y}) {found} instead of {wanted_depth}"));
            }
            let found = resolved.rgba8(x, y);
            if !close(found, wanted_color, tolerance) {
                resolves.push(format!("({x}, {y}) {found:?} instead of {wanted_color:?}"));
            }
        }
    }
    expect.pixels("the resolve attachment", colors);
    expect.pixels("the depth resolve attachment", depths);
    expect.pixels("the image resolved by vkCmdResolveImage", resolves);
}

pub fn descriptor_indexing(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let columns: [[[u8; 3]; 4]; 4] = [
        [[204, 51, 51], [204, 51, 51], [51, 204, 51], [51, 204, 51]],
        [[10, 20, 30], [40, 50, 60], [70, 80, 90], [100, 110, 120]],
        [[255, 128, 0], [0, 128, 255], [128, 0, 255], [128, 0, 255]],
        [[0, 128, 255]; 4],
    ];
    let mut mismatches = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let [red, green, blue] = columns[(x / 64) as usize][(y / 64) as usize];
            let wanted = [red, green, blue, 255];
            let found = color.rgba8(x, y);
            if found != wanted {
                mismatches.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
            }
        }
    }
    expect.pixels("the four columns", mismatches);
}

pub fn shader_objects(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let within = |x: u32, y: u32, (x0, y0, x1, y1): (u32, u32, u32, u32)| {
        (x0..x1).contains(&x) && (y0..y1).contains(&y)
    };
    let first = (32, 32, 160, 160);
    let second = (96, 96, 224, 224);
    let third = (16, 176, 80, 240);
    let mut mismatches = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let (wanted, tolerance) = match (within(x, y, first), within(x, y, second)) {
                (true, true) => ([128, 0, 128, 255], 1),
                (true, false) => ([255, 0, 0, 255], 0),
                (false, true) => ([16, 24, 160, 255], 1),
                (false, false) if within(x, y, third) => ([255, 48, 64, 255], 0),
                (false, false) => ([32, 48, 64, 255], 0),
            };
            let found = color.rgba8(x, y);
            if !close(found, wanted, tolerance) {
                mismatches.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
            }
        }
    }
    expect.pixels("the three rectangles", mismatches);
}

pub fn ray_query(expect: &mut Expect, images: &[OutputImage]) {
    let color = image(images, "color");
    let within = |x: u32, y: u32, (x0, y0, x1, y1): (u32, u32, u32, u32)| {
        (x0..x1).contains(&x) && (y0..y1).contains(&y)
    };
    let first = (32, 40, 96, 88);
    let second = (128, 120, 224, 200);
    let mut mismatches = Vec::new();
    for y in 0..color.height {
        for x in 0..color.width {
            let wanted = if within(x, y, first) {
                [255, 64, 128, 255]
            } else if within(x, y, second) {
                [64, 255, 191, 255]
            } else {
                [0, 0, 0, 255]
            };
            let found = color.rgba8(x, y);
            if !close(found, wanted, 1) {
                mismatches.push(format!("({x}, {y}) {found:?} instead of {wanted:?}"));
            }
        }
    }
    expect.pixels("the two visible instances", mismatches);
}

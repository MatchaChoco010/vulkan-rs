# vulkan-rs

Khronos の [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) から生成する、pure Rust の Vulkan バインディングです。

- C の API と 1 対 1 です。型、定数、command を registry にあるとおりに出します。
- 構造体は、指す先より長生きできません。寿命が足りないコードはコンパイルエラーになります。`build()` で寿命を消す手順はありません。
- 生成物は、C のヘッダと機械的に照合しています（構造体の大きさと offset、ビットフィールド、定数の値）。
- 現在の生成元は `vulkan-sdk-1.4.363.0` です。

| crate | 内容 |
| --- | --- |
| [`vulkan`](crates/vulkan) | バインディング本体 |
| [`vulkan-alloc`](crates/vulkan-alloc) | `VkDeviceMemory` の suballocator（TLSF） |
| [`vulkan-registry`](crates/vulkan-registry) | registry の dump と、その型 |
| [`vulkan-codegen`](crates/vulkan-codegen) | generator |

設計と API の規則は [docs/design.md](docs/design.md)、作り直しの経緯は [docs/audit.md](docs/audit.md)、開発の手順は [AGENTS.md](AGENTS.md) にあります。

## 使い始める

crates.io には公開していません。git か path の依存として使います。

```toml
[dependencies]
vulkan = { git = "https://github.com/MatchaChoco010/vulkan-rs", features = ["wayland"] }
vulkan-alloc = { git = "https://github.com/MatchaChoco010/vulkan-rs" }
```

instance と device を作る最小の例です。

```rust
use vulkan::{Entry, vk};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: この例は、呼んでいる command の valid usage を守っている。
    unsafe {
        let entry = Entry::load()?;

        let app = vk::ApplicationInfo::default()
            .application_name(c"example")
            .api_version(vk::API_VERSION_1_3);
        let instance =
            entry.create_instance(&vk::InstanceCreateInfo::default().application_info(&app), None)?;

        let physical_device = instance.enumerate_physical_devices()?[0];
        let properties = instance.get_physical_device_properties(physical_device);
        println!("{:?}", properties.device_name_as_c_str()?);

        // 構造体は、指している priorities、queues、vulkan13 より長生きできない。
        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(0)
            .queue_priorities(&priorities)];
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);
        let device = instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .push_next(&mut vulkan13),
            None,
        )?;
        let queue = device.get_device_queue(0, 0);
        println!("{queue:?}");

        device.destroy_device(None);
        instance.destroy_instance(None);
    }
    Ok(())
}
```

用途ごとの例は次の場所にあります。

| やりたいこと | 場所 |
| --- | --- |
| instance と device の作成、validation layer、`pNext` での照会 | [`crates/vulkan/tests/device.rs`](crates/vulkan/tests/device.rs) の `Gpu::create` |
| Wayland の surface と swapchain、色空間、present mode、HDR metadata | [`examples/wayland-dynamic-rendering`](examples/wayland-dynamic-rendering/src/main.rs) |
| dma-buf の export と import（DRM format modifier） | [`examples/dmabuf-roundtrip`](examples/dmabuf-roundtrip/src/main.rs) の `export_image` と `import_image` |
| DRM の device node の取得（`VK_EXT_physical_device_drm`） | 同じ example の `print_drm_nodes` |
| 画面なしでの描画と読み出し | [`examples/headless-triangle`](examples/headless-triangle/src/main.rs) |
| dynamic rendering、synchronization2、timeline semaphore | `crates/vulkan/tests/device.rs` |
| fence と semaphore の fd での受け渡し（sync file） | `crates/vulkan/tests/device.rs` |
| Y'CbCr 変換 | `crates/vulkan/tests/device.rs` |
| メモリの確保（`vulkan-alloc`）、export できるメモリ | [`crates/vulkan-alloc/tests/device.rs`](crates/vulkan-alloc/tests/device.rs) |

## examples

```sh
cargo run -p vulkan-headless-triangle
cargo run -p vulkan-compute-buffer
cargo run -p vulkan-dmabuf-roundtrip
cargo run -p vulkan-mesh-shader
cargo run -p vulkan-ray-tracing-pipeline
cargo run -p vulkan-descriptor-heap-14
cargo run -p vulkan-wayland-triangle
cargo run -p vulkan-wayland-dynamic-rendering
```

headless の example は、画像を `target/example-output/` に書きます。`dmabuf-roundtrip` 以外は、validation layer（`VK_LAYER_KHRONOS_validation`）が入っていないと起動しません。

`wayland-dynamic-rendering` は、環境変数で swapchain の設定を選べます。

| 変数 | 値 |
| --- | --- |
| `VULKAN_EXAMPLE_PRESENT_MODE` | `fifo`（既定）、`fifo_relaxed`、`immediate`（ティアリング）、`mailbox` |
| `VULKAN_EXAMPLE_COLOR_SPACE` | `srgb`（既定）、`hdr10`、`scrgb` |
| `VULKAN_EXAMPLE_FRAMES` | この数のフレームを描いて終了する |

lavapipe（ソフトウェア実装）で動かすには、ICD を指定します。

```sh
VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.x86_64.json cargo run -p vulkan-headless-triangle
```

## 版の付け方

workspace の版は `<semver>+<Vulkan-Headers の SDK タグ>` です（例: `0.3.0+vulkan-sdk-1.4.363.0`）。ルートの `Cargo.toml` の 1 か所で決め、全 crate が継承します。

- `+` の後ろは、生成元の SDK タグそのものです。
- 1.0 までは、Vulkan-Headers を更新したときと、API を非互換に変えたときに minor を上げます。Vulkan-Headers の更新では、暫定の拡張の名前が変わったり消えたりするので、互換とは限りません。
- 互換な修正は patch を上げます。

## Vulkan-Headers の更新

```sh
git submodule update --init
./tools/update-vulkan-headers
cargo fmt --check
cargo clippy --workspace --all-features --all-targets
cargo test --workspace --all-features
git commit -am "Update Vulkan bindings to $(git -C third_party/vulkan describe --tags --match 'vulkan-sdk-*')"
```

`tools/update-vulkan-headers` は、submodule を最新の SDK タグへ進め、版を上げ、再生成します。GitHub Actions（`.github/workflows/update-vulkan-headers.yml`）が週に 1 回これを行い、PR を作ります。

SDK タグ以外の commit から生成するとき（開発用）は、submodule を手で動かして `./tools/generate-vulkan --allow-non-sdk-tag` を使います。

## ライセンス

Apache-2.0

# vulkan-rs の設計

作り直した後の構成と、API の規則を説明します。作り直す前に何が問題だったかは [audit.md](audit.md) にあります。

## 目的

Khronos の Vulkan-Headers から生成する、pure Rust の Vulkan バインディングです。hizukuri（ゲームエンジン兼 Wayland compositor）の土台として使います。

方針は 3 つです。

1. **C の API と 1 対 1**。型、定数、command を、registry にあるとおりに出す。独自の抽象は足さない。
2. **寿命を型で追う**。構造体が指す先より長生きすることを、コンパイルエラーにする。
3. **C のヘッダと機械的に照合する**。生成物が正しいことを、人が読んで確かめるのではなく、テストで確かめる。

## 構成

```
crates/
  vulkan/            バインディング本体
    src/lib.rs         Entry / Instance / Device（手書き）
    src/vk/mod.rs      プラットフォームの型、基本の型、trait（手書き）
    src/vk/generated/  生成物
    tests/api.rs       API の形（Vulkan 実装なしで動く）
    tests/lifetime.rs  寿命の compile-fail テスト（trybuild、tests/ui/）
    tests/layout.rs    C のヘッダとの照合（tests/generated/ は生成物）
    tests/device.rs    実際の Vulkan 実装での結合テスト
  vulkan-alloc/      VkDeviceMemory の suballocator（TLSF）
  vulkan-registry/   registry の dump（vulkan-ir.json）と、その型
  vulkan-codegen/    generator
examples/            workspace の member。SPIR-V は commit してある
third_party/vulkan/  Vulkan-Headers（submodule、SDK タグに固定）
tools/
  generate-vulkan        再生成
  update-vulkan-headers  新しい SDK タグへの更新
  compile-shaders        examples の shader のコンパイル
```

## 生成の流れ

```
third_party/vulkan/registry/vk.xml, video.xml
        │  tools/generate-vulkan（Python。Khronos の VulkanObject で読む）
        ▼
crates/vulkan-registry/vulkan-ir.json     C の名前と registry の属性だけを持つ
        │  vulkan-codegen（Rust）
        ▼
crates/vulkan/src/vk/generated/*.rs       バインディング
crates/vulkan/tests/generated/layout.c    C 側の照合プログラム
crates/vulkan/tests/generated/layout_table.rs  Rust 側の照合表
        │  rustfmt
        ▼
       commit
```

### なぜ Python と Rust の 2 段なのか

`vk.xml` の解釈（alias、拡張による列挙値の追加、`api="vulkan"` の絞り込み、`len` や `optional` の読み取り）は、Khronos が `Vulkan-Headers/registry` に置いている `VulkanObject` に任せます。これは Khronos 自身のツールが使っている実装で、`vk.xml` の書式が変わっても Khronos の側で追従されます。自分で XML を読むと、その追従を自分でやることになります。

Python のスクリプトは `VulkanObject` を JSON に写すだけで、判断はしません。**generator が実際に使う属性だけ**を写します。使わない属性まで写すと、Khronos がその属性の名前を変えたときに、無関係な理由で生成が止まります（実際に `Member.objectType` でそうなりました）。逆に、使う属性が消えたときは、既定値で黙って続けずに例外で止まります。

### generator の中

| module | 役割 |
| --- | --- |
| `naming` | C の識別子から Rust の識別子への対応 |
| `model` | 型の検索、alias の解決、C の型から Rust の型への対応、寿命の要否、command の読み込み先 |
| `emit::basic` | 定数、ハンドル、列挙型、フラグ、関数ポインタ型、拡張名 |
| `emit::structs` | 構造体と共用体、setter、ビットフィールド、`push_next` |
| `emit::commands` | 関数ポインタ表と command の wrapper |
| `emit::layout` | C のヘッダとの照合に使う C のプログラムと Rust の表 |

generator は、registry に知らない形が現れたら panic します。たとえば次の場合です。

- registry にもプラットフォームの型の一覧（`model::EXTERNAL_TYPES`）にもない型
- `crates/vulkan/Cargo.toml` に feature がないプラットフォーム
- 32 bit でないビットフィールド
- 同じ Rust の名前になり、値が違う 2 つの定数

黙って誤ったコードを出すより、更新のときに止まって人（またはエージェント）が判断する方を選んでいます。

## API の規則

### 名前

| C | Rust |
| --- | --- |
| `VkBufferCreateInfo` | `vk::BufferCreateInfo` |
| `VkResult` | `vk::VkResult`（`vk::Result<T>` は結果の型） |
| `vkCreateBuffer` | `device.create_buffer(..)` と `device.create_buffer_raw(..)` |
| `pQueueCreateInfos`（member） | field `p_queue_create_infos`、setter `queue_create_infos` |
| `VK_PRESENT_MODE_IMMEDIATE_KHR` | `vk::PresentModeKHR::IMMEDIATE` |
| `VK_COLOR_SPACE_HDR10_ST2084_EXT` | `vk::ColorSpaceKHR::HDR10_ST2084_EXT` |
| `VK_IMAGE_USAGE_SAMPLED_BIT` | `vk::ImageUsageFlags::SAMPLED`、`vk::ImageUsageFlagBits::SAMPLED` |
| `VK_ACCESS_2_SHADER_READ_BIT` | `vk::AccessFlags2::SHADER_READ` |
| `VK_SAMPLE_COUNT_1_BIT` | `vk::SampleCountFlagBits::_1` |
| `VK_KHR_SWAPCHAIN_EXTENSION_NAME` | `vk::KHR_SWAPCHAIN_NAME`（`&CStr`） |
| `VK_API_VERSION_1_3` | `vk::API_VERSION_1_3` |

列挙値は、型名の接頭辞を取ります。型と同じ vendor tag は取り、違う tag は残します。フラグは `_BIT` も取ります。数字で始まる名前には `_` を付けます。拡張名の定数は C のマクロ名に従うので、`VK_EXT_swapchain_colorspace` は `vk::EXT_SWAPCHAIN_COLOR_SPACE_NAME` です。

`FlagBits`（1 bit を表す型）と `Flags`（集合）は、C と同じく別の型です。`FlagBits | FlagBits` は `Flags` になり、`FlagBits` は `.into()` で `Flags` になります。

### 構造体

```rust
let priorities = [1.0];
let queues = [vk::DeviceQueueCreateInfo::default()
    .queue_family_index(0)
    .queue_priorities(&priorities)];
let names = [vk::CStrPtr::new(vk::KHR_SWAPCHAIN_NAME)];
let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default().dynamic_rendering(true);
let info = vk::DeviceCreateInfo::default()
    .queue_create_infos(&queues)
    .enabled_extension_names(&names)
    .push_next(&mut vulkan13);
```

- `Default` は `sType` を設定し、ポインタを null、残りを 0 にします。0 が有効な値でも中立な値でもない次の member だけ、別の値にします。`srcQueueFamilyIndex` / `dstQueueFamilyIndex` は `QUEUE_FAMILY_IGNORED`、`VkImageCreateInfo` の `mipLevels` / `arrayLayers` は 1、`VkSampleCountFlagBits` の member は `_1`。
- setter は `self` を受け取って返します。`build()` はありません。
- **ポインタを持ち得る構造体だけ**が寿命 `'a` を持ちます。`vk::Extent2D` のような値だけの構造体は寿命を持たず、構造体リテラルで書け、`PartialEq`（float がなければ `Eq` と `Hash` も）を実装します。
- field はすべて公開です。Vulkan が書き込んだ結果を読むためと、setter がない形のためです。ポインタの field に直接書くと、寿命は追われません。

setter が受け取る型は、member の形で決まります。

| member | setter の引数 |
| --- | --- |
| 値、ハンドル、列挙型、構造体 | そのままの型 |
| `VkBool32` | `bool` |
| `const T* p` + `count` | `&'a [T]`（count も設定する） |
| `T* p` + `count`（出力用の配列） | `&'a mut [T]` |
| `const void* p` + `size` | `&'a [u8]` |
| `const T* p`（1 個） | `&'a T` |
| `const char* p` | `&'a CStr` |
| `const char* const* pp` + `count` | `&'a [vk::CStrPtr<'a>]` |
| `const T* const* pp` + `count` | `&'a [&'a T]`（`pGeometries` と名前が重なる `ppGeometries` などは `..._ptrs`） |
| `void*`、プラットフォームの不透明な型 | 生ポインタ |
| 固定長の配列 | 配列そのもの |
| C のビットフィールド | `u32` またはフラグの型。getter は `get_...` |

補足です。

- 同じ count を共有する配列（`SubmitInfo` の `wait_semaphores` と `wait_dst_stage_mask` など）は、長さが違うと panic します。
- count の setter も残してあります。配列を渡さずに数だけ設定する場合（dynamic state の `viewport_count` など）に使います。
- 長さが式で決まる配列（`pSampleMask` など）は、スライスを受け取ってポインタだけを設定します。
- 固定長の `char` 配列には `..._as_c_str()`、count を伴う固定長の配列には `..._as_slice()` があります（`device_name_as_c_str()`、`memory_types_as_slice()`）。
- ビットフィールドは、連続するものを `bitfield0`、`bitfield1`、… という `u32` の field にまとめます。最初の member が下位 bit です。

### `push_next`

```rust
pub fn push_next<T: Extends<Self> + TaggedStructure>(mut self, next: &'a mut T) -> Self
```

- registry が「この root に付けられる」としている構造体だけを受け取ります。
- `next` を chain の先頭に入れます。**`next.p_next` は上書きします**。あらかじめ chain を組んだ構造体を渡しても、その先は捨てられます。1 個ずつ push してください。
- `&'a mut` で借りるので、同じ構造体を 2 回入れることはできず、root が生きている間は `next` に触れません。照会の結果は、root を使い終えてから読みます。

```rust
let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut vulkan12);
instance.get_physical_device_features2(physical_device, &mut features);
let anisotropy = features.features.sampler_anisotropy; // root はここまで
let timeline = vulkan12.timeline_semaphore;            // ここから読める
```

chain の中に出力用の配列がある場合（`DrmFormatModifierPropertiesListEXT` など）は、1 回目は配列なしで呼んで数を得て、2 回目に `&mut [T]` を渡します。`examples/dmabuf-roundtrip` の `format_modifiers` が例です。

### command

`Entry`、`Instance`、`Device` の method です。どれに付くかは、registry の区分で決まります。`VkInstance` / `VkDevice` を第 1 引数に取る command は、その引数を省きます。`VkPhysicalDevice`、`VkQueue`、`VkCommandBuffer` は明示的に渡します。

各 command に 2 つの method があります。

- `foo_raw`: C の signature そのまま。
- `foo`: 次の表のとおりに形を変えたもの。

| C の引数 | `foo` での形 |
| --- | --- |
| `count` + `const T* p` | `&[T]`（同じ count を共有する配列は、長さが違うと panic） |
| `const T* p` | `&T`。省略できるなら `Option<&T>` |
| `const char* p` | `&CStr` |
| `VkBool32` | `bool` |
| `const float v[4]` | `&[f32; 4]` |
| `size` + `void* p` | `&mut [u8]` |
| `void*`、プラットフォームの不透明な型、`T**` | 生ポインタのまま |
| 最後の `T* p`（出力 1 個） | 戻り値 |
| 最後の `T* p` で、`T` が `pNext` を持つ | `&mut T`（呼ぶ側が chain を組む） |
| `uint32_t* pCount` + `T* p`（列挙） | `Vec<T>`。`VK_INCOMPLETE` なら数を聞き直す |
| 入力の配列と同じ長さの出力の配列 | `Vec<T>` |
| 入力の構造体の member が長さの出力の配列 | `Vec<T>` |

戻り値の規則です。

- `VkResult` を返す command は `vk::Result<T>`。負の値が `Err` です。
- 成功コードが `VK_SUCCESS` 以外にもある command は、`Ok` の中に `VkResult` を入れて返します。`device.wait_for_fences(..)` は `Ok(VkResult::SUCCESS)` か `Ok(VkResult::TIMEOUT)` です。
- 列挙は `VK_INCOMPLETE` を内部で処理するので、`Vec<T>` だけを返します。
- 特例は 3 種類です。
  - `acquire_next_image_khr`: `Result<Option<(u32, bool)>>`。`None` は `TIMEOUT` / `NOT_READY`（index は書かれていない）、`bool` は `SUBOPTIMAL`。
  - `queue_present_khr`: `Result<bool>`。`true` は `SUBOPTIMAL`。
  - `create_graphics_pipelines` など: `Result<Vec<Pipeline>, (Vec<Pipeline>, VkResult)>`。失敗しても、途中まで作られたハンドルを返します。
- acceleration structure の build（`ppBuildRangeInfos`）は、registry の属性から形を決められないので、`&[&[T]]` を受け取る wrapper を個別に書いています。

**すべての command は `unsafe fn` です**。Vulkan の valid usage（ハンドルが有効か、外部同期が守られているか、構造体の内容が規則に合うか）は型で表せません。寿命の検査が保証するのは「構造体が、指す先より長生きしない」ことだけです。

### 関数ポインタの読み込み

- `Entry::load()` が loader（`libvulkan.so.1`）を開き、global command を読みます。
- `entry.create_instance(..)` が instance を作り、**すべての** instance command を `vkGetInstanceProcAddr` で読みます。
- `instance.create_device(..)` が device を作り、**すべての** device command を `vkGetDeviceProcAddr` で読みます。

拡張ごとに loader の型を作る必要はありません。有効にしていない拡張の command は、表の中で `None` になります。`None` の command を呼ぶと panic します（メッセージに command 名が出ます）。事前に確かめるには `device.fp().create_swapchain_khr.is_some()` を見ます。

`Instance` と `Device` は、ハンドルと関数ポインタ表を持つだけの値で、`Clone` です。drop しても Vulkan のオブジェクトは破棄されません。`destroy_instance` / `destroy_device` を呼びます。loader への参照を共有するので、`Entry` を先に drop しても関数ポインタは有効です。

### feature

| feature | 内容 |
| --- | --- |
| `std`（既定） | `Entry::load`。なければ `no_std` + `alloc` |
| `debug`（既定） | 構造体の `Debug` |
| `linked` | `Entry::linked`（loader を link する） |
| `wayland`、`xcb`、`xlib`、`win32`、… | プラットフォームの型と command。名前は registry の platform 名 |
| `beta` | 暫定の拡張（`VK_ENABLE_BETA_EXTENSIONS`） |

hizukuri は `features = ["wayland"]` を足します。`VK_KHR_display` や DRM 関連はプラットフォームの feature を必要としません。

### `Send` と `Sync`

ハンドル、構造体、`Entry` / `Instance` / `Device` は `Send + Sync` です。Vulkan の外部同期の規則（同じ `VkCommandPool` を 2 つのスレッドから同時に使わない、など）は、command を呼ぶ側の責任です。

## テスト

| テスト | 確かめること | 必要なもの |
| --- | --- | --- |
| `vulkan-codegen` の単体テスト | 命名、型の対応、寿命の要否、各 emitter の出力 | なし |
| `tests/api.rs` | 生成された API の形と値 | なし |
| `tests/lifetime.rs` | 寿命が足りないコードがコンパイルエラーになる | なし |
| `tests/layout.rs` | 大きさ、offset、ビットフィールド、定数が C と一致する | submodule、C コンパイラ |
| `tests/device.rs` | 実際の Vulkan 実装で動く。validation のエラーが出ない | Vulkan 1.3 の device |
| `vulkan-alloc` の単体テスト | 分割と解放の処理 | なし |
| `vulkan-alloc/tests/device.rs` | block の共有の規則、export | Vulkan 1.2 の device |

`layout.rs` と `device.rs` は、必要なものがなければ skip します。`VULKAN_RS_REQUIRE_LAYOUT_TEST` と `VULKAN_RS_REQUIRE_DEVICE_TEST` を設定すると、skip せずに失敗します。CI は両方を設定します。

lavapipe で動かすには、ICD を指定します。

```sh
VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.x86_64.json cargo test --workspace --all-features
```

## 版の付け方

workspace の版は `<semver>+<Vulkan-Headers の SDK タグ>` です（例: `0.3.0+vulkan-sdk-1.4.363.0`）。`Cargo.toml` の `[workspace.package]` の 1 か所で決め、全 crate が継承します。

- `+` の後ろは、生成元の SDK タグそのものです。`tests/api.rs` が、`vk::HEADERS_TAG` と一致することを確かめます。
- 1.0 までは、次のときに minor を上げます。
  - Vulkan-Headers を更新したとき。暫定の拡張の名前が変わったり消えたりするので、互換とは限らない。
  - この crate の API を非互換に変えたとき。
- 互換な修正は patch を上げます。
- 1 つの commit に複数の SDK タグが付いている場合（`1.4.350.0` と `1.4.350.1`）は、新しい方を使います。

以前は、4 つの crate の `Cargo.toml` を手で書き換える手順でした。今は `tools/update-vulkan-headers` が書き換えます。

## 更新の手順

```sh
./tools/update-vulkan-headers    # submodule を最新の SDK タグへ。版を上げ、再生成する
cargo fmt --check
cargo clippy --workspace --all-features --all-targets
cargo test --workspace --all-features
```

`.github/workflows/update-vulkan-headers.yml` が、週に 1 回これを行い、PR を作ります。検査が失敗した場合は draft の PR になります。generator が止まった場合は workflow が失敗します。

更新で止まりやすい箇所と、直し方です。

| 症状 | 直し方 |
| --- | --- |
| `unknown C type` で generator が panic | `vk/mod.rs` に型を宣言し、`model::EXTERNAL_TYPES` に足す |
| `add these features to crates/vulkan/Cargo.toml` | 新しいプラットフォームの feature を足す |
| `AttributeError`（Python） | Khronos が `VulkanObject` の属性を変えた。`tools/generate-vulkan` と `vulkan-registry` を合わせる |
| `already has value`（定数名の衝突） | `naming` の規則を見直す |
| `layout.rs` が失敗 | generator の誤り。新しい形の member を調べる |
| examples がコンパイルできない | 暫定の拡張の名前が変わった。examples を直す |

## 残っている課題

- **format の情報**。registry は、format ごとの block の大きさ、plane の数、component の bit 数を持っています。compositor は dma-buf の format を扱うときにこれが要ります。`vk::Format` に method として生やせます。
- **拡張の依存関係**。registry の `depends` を使えば、「この拡張を有効にするには何が要るか」を定数として出せます。
- **`VK_KHR_display` で画面を取得する経路の実機確認**。DRM master が要るので、TTY から動かす example が必要です。
- **ティアリングと HDR の実機確認**。対応する compositor の上で、`examples/wayland-dynamic-rendering` を動かして確かめます。hizukuri 自身が compositor になれば、そこで確かめられます。
- **Linux x86_64 以外での照合**。`layout.rs` は実行した環境の ABI を見るので、CI に aarch64 や Windows を足せば、そのまま照合できます。
- **プラットフォーム固有の構造体の照合**。プラットフォームのヘッダが要ります。Wayland だけなら、`wayland-client.h` がある環境で足せます。
- **`vulkan-alloc` の drop 時の panic**。確保が残ったまま drop すると panic します。ログに出して続ける選択肢を検討します。

# vulkan-rs の設計

作り直した後の構成と、API の規則を説明します。

- 作り直す前に何が問題だったか: [audit.md](audit.md)
- スクリプトの誤りで落ちない層を、どこに、どう作るか: [safety-layer.md](safety-layer.md)

## 目的

Khronos の Vulkan-Headers から生成する、pure Rust の Vulkan バインディングです。hizukuri（ゲームエンジン兼 Wayland compositor）の土台として使います。

hizukuri は Vulkan 1.4 と `VK_EXT_descriptor_heap` を前提にします。vulkan-rs は registry の全てを生成するので、従来の descriptor set の API も残りますが、手書きの層、`vulkan-alloc`、結合テストは、descriptor set を前提にしていません（「descriptor heap」の節）。

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
    src/vk/format.rs   vk::Format::info が返す型（手書き。表は生成物）
    src/vk/extension.rs  vk::ExtensionInfo と依存の評価（手書き。表は生成物）
    src/vk/generated/  生成物
    tests/api.rs       API の形（Vulkan 実装なしで動く）
    tests/lifetime.rs  寿命の compile-fail テスト（trybuild、tests/ui/）
    tests/layout.rs    C のヘッダとの照合（tests/generated/ は生成物）
    tests/device.rs    実際の Vulkan 実装での結合テスト（tests/shaders/ に SPIR-V）
  vulkan-alloc/      VkDeviceMemory の suballocator（TLSF）
  vulkan-registry/   registry の dump（vulkan-ir.json）と、その型
  vulkan-codegen/    generator
examples/            workspace の member。SPIR-V は commit してある
third_party/vulkan/  Vulkan-Headers（submodule、SDK タグに固定）
tools/
  generate-vulkan        再生成
  update-vulkan-headers  新しい SDK タグへの更新
  compile-shaders        examples とテストの shader のコンパイル
rust-toolchain.toml  Rust の版の固定（rustfmt の出力と trybuild の期待値を揃える）
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
| `emit::tables` | format の表（`Format::info`）と拡張の表（`EXTENSIONS`） |
| `emit::layout` | C のヘッダとの照合に使う C のプログラムと Rust の表 |

generator は、registry に知らない形が現れたら panic します。たとえば次の場合です。

- registry にもプラットフォームの型の一覧（`model::EXTERNAL_TYPES`）にもない型
- `crates/vulkan/Cargo.toml` に feature がないプラットフォーム
- 32 bit でないビットフィールド
- 同じ Rust の名前になり、値が違う 2 つの定数
- `<format>` の記述がない format、知らない数値形式
- 括弧が合わない依存の式

黙って誤ったコードを出すより、更新のときに止まって人（またはエージェント）が判断する方を選んでいます。

## API の規則

### 名前

| C | Rust |
| --- | --- |
| `VkBufferCreateInfo` | `vk::BufferCreateInfo` |
| `VkResult` | `vk::VkResult`（`vk::Result<T>` は結果の型） |
| `vkCreateBuffer` | `device.create_buffer(..)` と `device.create_buffer_raw(..)` |
| `pQueueCreateInfos`（member） | field `p_queue_create_infos`、setter `queue_create_infos` |
| `type`（member、引数） | `ty`（`vk::DescriptorPoolSize::default().ty(..)`） |
| `VK_PRESENT_MODE_IMMEDIATE_KHR` | `vk::PresentModeKHR::IMMEDIATE` |
| `VK_COLOR_SPACE_HDR10_ST2084_EXT` | `vk::ColorSpaceKHR::HDR10_ST2084_EXT` |
| `VK_IMAGE_USAGE_SAMPLED_BIT` | `vk::ImageUsageFlags::SAMPLED`、`vk::ImageUsageFlagBits::SAMPLED` |
| `VK_ACCESS_2_SHADER_READ_BIT` | `vk::AccessFlags2::SHADER_READ` |
| `VK_SAMPLE_COUNT_1_BIT` | `vk::SampleCountFlagBits::_1` |
| `VK_KHR_SWAPCHAIN_EXTENSION_NAME` | `vk::KHR_SWAPCHAIN_NAME`（`&CStr`） |
| `VK_API_VERSION_1_3` | `vk::API_VERSION_1_3` |

列挙値は、型名の接頭辞を取ります。型と同じ vendor tag は取り、違う tag は残します。フラグは `_BIT` も取ります。

接頭辞を取った結果が数字で始まる名前（`VK_IMAGE_TYPE_2D`、`VK_SAMPLE_COUNT_1_BIT`）には、`_` を付けます（`vk::ImageType::_2D`）。`TYPE_2D` のような語を補う方式は採りませんでした。補う語を型ごとに決める必要があり、機械的な規則でなくなるためです。

Rust の予約語と重なる名前のうち、`type` だけは `ty` にします。Vulkan に多く現れ、`r#type` と毎回書くのは煩わしいためです。ほかの予約語が現れた場合は `r#` を付けます（現在の registry にはありません）。拡張名の定数は C のマクロ名に従うので、`VK_EXT_swapchain_colorspace` は `vk::EXT_SWAPCHAIN_COLOR_SPACE_NAME` です。

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

### format の情報

C のヘッダには、format ごとの texel の大きさや plane の構成がありません。registry の `<formats>` から生成します。

```rust
let info = vk::Format::G8_B8R8_2PLANE_420_UNORM.info().unwrap();
assert_eq!(info.plane_count(), 2);
assert_eq!(info.planes[1].compatible, vk::Format::R8G8_UNORM);
let chroma = info.plane_extent(1, vk::Extent2D { width: 1920, height: 1080 }); // 960x540
let aspects = vk::Format::D24_UNORM_S8_UINT.info().unwrap().aspects();      // DEPTH | STENCIL
```

`FormatInfo` が持つのは、互換性の class、block の大きさと texel 数、block の範囲、pack の幅、chroma の間引き、圧縮の方式、component（名前、bit 数、数値形式、属する plane）、plane（幅と高さの除数、同じ配置を持つ単一 plane の format）です。`info` は `const fn` なので、定数の計算に使えます。`UNDEFINED` と、このバインディングが知らない値には `None` を返します。

### 拡張の依存関係

`vk::ExtensionInfo::get(name)` が、拡張の種類（instance か device か）、依存の式、昇格先を返します。`vk::EXTENSIONS` は、プラットフォームの feature に関係なく、全ての拡張を持ちます。

```rust
let heap = vk::ExtensionInfo::get(vk::EXT_DESCRIPTOR_HEAP_NAME).unwrap();
// Vulkan 1.3 では VK_KHR_maintenance5 が要る。1.4 では不要。
assert!(!heap.dependencies_met(vk::API_VERSION_1_3, |_| false));
assert!(heap.dependencies_met(vk::API_VERSION_1_3, |name| name == vk::KHR_MAINTENANCE_5_NAME));
assert!(heap.dependencies_met(vk::API_VERSION_1_4, |_| false));
println!("{}", heap.depends.unwrap()); // registry の記法で表示する
```

- 式は `Dependency::{Version, Extension, All, Any}` の木です。registry の `depends` の文法（`+` が and、`,` が or、優先順位は同じで左結合）のとおりに解釈します。
- 依存先の拡張が、使っている core の版に昇格済みなら、依存は満たされたものとします。仕様の一般則に従っています。
- device の拡張が instance の拡張に依存することがあるので、`enabled` は両方の一覧について答える必要があります。
- command や構造体が「どの拡張で提供されるか」は、doc comment にだけ出しています。表としては未生成です（[safety-layer.md](safety-layer.md) の 3.1）。

### descriptor heap

`VK_EXT_descriptor_heap` では、descriptor set layout、pipeline layout、descriptor pool、descriptor set がなくなります。

- heap は、`DESCRIPTOR_HEAP_EXT` の usage を持つ buffer です。resource 用と sampler 用の 2 つを bind します。
- descriptor は、`write_resource_descriptors_ext` / `write_sampler_descriptors_ext` が、ホストのメモリに書きます。書き込み先は `vk::HostAddressRangeEXT::default().address(&mut bytes)` で、スライスとして渡します。heap の外には書けません。
- pipeline は `vk::PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT` を付けて、layout なしで作ります。
- shader は、heap の中の番号で資源を引きます。番号に descriptor の大きさ（`bufferDescriptorSize` など）を掛けたものが、heap の中の offset です。番号は `cmd_push_data_ext` などで渡します。
- driver が使う範囲（`minResourceHeapReservedRange`）を、heap の中に確保して `BindHeapInfoEXT` で伝えます。

動く例は 2 つあります。どちらも RADV と lavapipe（Mesa 26.2）で、validation layer のエラーなしに通ります。

- `crates/vulkan/tests/device.rs` の `descriptor_heap_draws_through_resource_and_sampler_heaps`。texture、sampler、storage buffer を heap に書き、描画して、結果の画素を確かめます。最小の形です。
- `examples/descriptor-heap-14`。複数の material と texture を、buffer の中の番号から間接的に引きます。

descriptor set を前提にした部分の洗い出しの結果です。

| 場所 | 結果 |
| --- | --- |
| `crates/vulkan` の手書きの層 | なし |
| `vulkan-alloc` | なし。メモリの確保だけを扱う。heap 用の buffer は、`memory_allocate_flags: DEVICE_ADDRESS` を付けて普通に確保する |
| generator | なし。特定の拡張に依存する処理はない |
| 結合テスト | なし |
| examples | `compute-buffer`、`mesh-shader`、`ray-tracing-pipeline` の 3 つが descriptor set を使う。バインディングの例としては動くが、hizukuri の手本にはしない |

寿命の規則は、descriptor heap の構造体にもそのまま効きます。書き込み元（`ResourceDescriptorInfoEXT` → 共用体 → `ImageDescriptorInfoEXT` → `ImageViewCreateInfo`）と、書き込み先（`HostAddressRangeEXT`）のどちらにも compile-fail テストがあります。

この作業で、生成の誤りを 1 つ直しました。`vkGetImageOpaqueCaptureDataEXT` は、書き込み先を示す `VkHostAddressRangeEXT` の配列を呼ぶ側が渡す command ですが、wrapper が配列を「出力」と判定して、null の範囲の `Vec` を返していました。ポインタを持つ構造体の配列は、出力として扱わないようにしました。

## `vulkan-alloc`

`VkDeviceMemory` を、TLSF で分割して使う allocator です。使い方と注意は、crate の文書（`crates/vulkan-alloc/src/lib.rs` の冒頭）にあります。設計上の決まりは次のとおりです。

- **確保が残ったまま drop しても、panic しない**。残っている確保を、`AllocationCreateDesc::name` の名前つきで `log` crate の `error` に出し、その `VkDeviceMemory` は解放しません。まだ使われている buffer や image の裏にあるかもしれず、解放すると、漏れがドライバの中の use-after-free に変わるためです。メモリは device の破棄まで残ります。
- **明示的に全部を解放するのは `Allocator::free_all`**。資源を破棄した後で呼びます。スクリプトを動かすホストの終了処理は、これを使います。`Allocator::live_allocations` で、残っている確保を一覧できます。
- **`map` は guard を返す**。guard の drop が unmap です。`map_memory` / `unmap_memory` の組とは別の経路で、混ぜません。範囲外の指定で panic しない `try_byte_range` があります。
- **破棄の順序は、型では保証しない**。`vulkan::Device` はハンドルと関数ポインタ表で、所有者ではありません。allocator が device の clone を持っていても、`destroy_device` を先に呼ぶことは防げません。順序（資源、確保、allocator、device）は呼ぶ側が守ります。allocator の drop は Vulkan を呼ばないので、device の破棄の後に drop しても害はありません。所有と破棄の順序を型で保証するのは、上に載せる層の役割です。
- allocate flags が違う確保は、同じ `VkDeviceMemory` を共有しません。export するメモリ（`export_handle_types`）と dedicated の確保は、専用の `VkDeviceMemory` になります。import するメモリは扱いません。

## テスト

| テスト | 確かめること | 必要なもの |
| --- | --- | --- |
| `vulkan-codegen` の単体テスト | 命名、型の対応、寿命の要否、各 emitter の出力 | なし |
| `tests/api.rs` | 生成された API の形と値 | なし |
| `tests/lifetime.rs` | 寿命が足りないコードがコンパイルエラーになる | なし |
| `tests/layout.rs` | 大きさ、offset、ビットフィールド、定数が C と一致する | submodule、C コンパイラ |
| `tests/device.rs` | 実際の Vulkan 実装で動く。validation のエラーが出ない。descriptor heap での描画を含む | Vulkan 1.3 の device |
| `vulkan-alloc` の単体テスト | 分割と解放の処理 | なし |
| `vulkan-alloc/tests/device.rs` | block の共有の規則、export、残った確保の報告、`free_all` | Vulkan 1.2 の device |

`layout.rs` と `device.rs` は、必要なものがなければ skip します。`VULKAN_RS_REQUIRE_LAYOUT_TEST` と `VULKAN_RS_REQUIRE_DEVICE_TEST` を設定すると、skip せずに失敗します。CI は両方を設定します。

結合テストは、ドライバが対応していない拡張のテストを、個別に skip します。CI（Ubuntu 24.04、Mesa 25.2 の lavapipe）では、何が skip されたかを job の summary に出します。descriptor heap のテストは、Mesa 26 以降の lavapipe で動きます。

Rust の版は `rust-toolchain.toml` で固定しています。生成物の整形（rustfmt）と、compile-fail テストが照合するコンパイラのメッセージが、版によって変わるためです。上げるときは、`./tools/generate-vulkan` と `TRYBUILD=overwrite cargo test -p vulkan --test lifetime` をやり直し、差分を確かめます。

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

- **registry の表の追加**。command ごとの要件（拡張、version、queue の種類、render の内外）、列挙値が既知かどうか、SPIR-V の capability の表、`externsync` の表。検査の層が必要とするものから足します（[safety-layer.md](safety-layer.md) の 3.1）。
- **descriptor set を使う examples の扱い**。`compute-buffer`、`mesh-shader`、`ray-tracing-pipeline` を descriptor heap に書き直すか、消すか。
- **ティアリングと HDR の実機確認**。対応する compositor の上で、`examples/wayland-dynamic-rendering` を動かして確かめます。hizukuri 自身が compositor になれば、そこで確かめられます。
- **Linux x86_64 以外での照合**。`layout.rs` は実行した環境の ABI を見るので、CI に aarch64 や Windows を足せば、そのまま照合できます。
- **プラットフォーム固有の構造体の照合**。プラットフォームのヘッダが要ります。Wayland だけなら、`wayland-client.h` がある環境で足せます。
- **`VK_KHR_display` で画面を取得する経路**。hizukuri は DRM/KMS を直接扱うので、example は作りません。バインディングとしては、signature の確認と、列挙が動くことまでを確かめています。

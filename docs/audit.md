# vulkan-rs 監査結果

- 対象: commit `b5f0706`（`0.2.0+vulkan-sdk-1.4.350.0`）
- 実施日: 2026-11-12
- 環境: Fedora 44、rustc 1.96.0、Radeon RX 9060 XT（RADV、Mesa 26.2.6）、lavapipe（Mesa 26.2.6）、niri（Wayland）

この文書は、作り直す前の状態を調べた結果と、見つけた問題をそれぞれどうしたかを記録します。作り直した後の設計は [design.md](design.md) にあります。

## 結論

公開 API の方向（ash と同じく、構造体自体が寿命を持ち `build()` がない）は正しかったので、そのまま残しました。一方で、generator が出すコードには C のヘッダと食い違う箇所があり、generator 自体も 1 ファイルに条件分岐を積み上げた作りで直しにくかったため、**generator は全面的に書き直し、生成コードはすべて作り直しました**。手書きの `Entry` / `Instance` / `Device` と `vulkan-alloc` は、必要な箇所だけ直しました。

作り直しを選んだ理由は次の 3 つです。

1. 生成コードに ABI の誤りがあった（A1、A2、A3）。部分的な修正では、ほかに同種の誤りがないことを示せない。C のヘッダと機械的に照合する仕組みを入れ、それを通るように generator を作り直す方が確実だった。
2. 命名の誤り（A5）と、不要な寿命パラメータ（B6）を直すと、生成物のほぼ全行が変わる。利用側の変更量は部分修正でも作り直しでも変わらない。
3. 利用者がまだいない。hizukuri にはコードがなく、このリポジトリの examples だけが利用者だった。

## A. 生成コードの正しさ

### A1. C のビットフィールドを無視していた（重大）

C のビットフィールド（`uint32_t x : 1`）を、1 個ずつ独立した `u32` として出していました。大きさも、後ろに続く member の位置も C と食い違います。

| 型 | C の大きさ | 旧 Rust の大きさ |
| --- | --- | --- |
| `StdVideoH264SpsFlags` | 4 | 64 |
| `StdVideoH264SequenceParameterSet` | 88 | 152 |
| `VkAccelerationStructureMatrixMotionInstanceNV` | 112 | 120 |
| `VkAccelerationStructureSRTMotionInstanceNV` | 144 | 152 |

`VkAccelerationStructureInstanceKHR` だけは手書きの特例で正しく出ていました。ビットフィールドを持つ構造体は 45 個あり（1.4.363 時点）、そのほとんどが `vk_video` の `*Flags` です。Vulkan Video を使うと、ドライバが誤った位置を読みます。

- 確かめ方: 同じ型の `sizeof` を C と Rust で出力して比べた。
- 対応: 連続するビットフィールドを `u32` の格納単位にまとめ、member ごとに setter と getter を出すようにした。特例は廃止した。

### A2. 配列の引数を値で渡していた（重大）

C では、配列の引数はポインタとして渡されます。旧コードは `vkCmdSetBlendConstants` の関数ポインタ型を `fn(CommandBuffer, [f32; 4])` としていて、配列を値で渡していました。呼び出すと、ドライバは引数をポインタとして読みます。該当するのは `vkCmdSetBlendConstants`、`vkCmdSetFragmentShadingRateKHR`、`vkCmdSetFragmentShadingRateEnumNV` の 3 つです。

- 確かめ方: 旧生成コードの型を読んだ。
- 対応: 関数ポインタ型では `*const [f32; 4]`、wrapper では `&[f32; 4]` にした。

### A3. command の読み込み先を、第 1 引数の型名で決めていた

第 1 引数が `VkDevice` / `VkQueue` / `VkCommandBuffer` なら device、`VkInstance` / `VkPhysicalDevice` なら instance、それ以外は entry（global）としていました。`vkGetExternalComputeQueueDataNV` は第 1 引数が device の子ハンドルなので、global command として `EntryFn` に入り、`vkGetInstanceProcAddr(NULL, ...)` で読まれていました。

- 対応: registry が持つ instance / device の区別を使うようにした。

### A4. 「指す先が null でもよい」を「ポインタが null でもよい」と取り違えていた

registry の `optional="false,true"` は「ポインタは必須、指す先の値は 0 でもよい」という意味です。旧コードはこれを「ポインタが省略できる」と扱い、`cmd_bind_vertex_buffers(cb, 0, Some(&buffers), &offsets)` のように不要な `Option` を要求していました。

- 対応: ポインタ自体が省略できるときだけ `Option` にした。

### A5. vendor tag を持つ型の定数名から、型名の接頭辞が取れていなかった

`vk::PresentModeKHR::PRESENT_MODE_IMMEDIATE_KHR`、`vk::ColorSpaceKHR::COLOR_SPACE_SRGB_NONLINEAR_KHR`、`vk::PipelineStageFlags2::PIPELINE_STAGE_2_COPY` のようになっていました。型名の末尾の vendor tag を考慮せずに接頭辞を比べていたためです。

- 対応: `vk::PresentModeKHR::IMMEDIATE`、`vk::ColorSpaceKHR::SRGB_NONLINEAR`、`vk::PipelineStageFlags2::COPY` になるようにした。型と同じ vendor tag は取り、違う tag は残す（`vk::ColorSpaceKHR::HDR10_ST2084_EXT`）。

### A6. 列挙値とフラグの alias がなかった

`VK_PRESENT_MODE_FIFO_LATEST_READY_EXT` のような、昇格前の名前に対応する定数がありませんでした。

- 対応: alias も定数として出す。名前が衝突し、値が違う場合は生成を止める。

### A7. ハンドルと構造体が `Send` でも `Sync` でもなかった

dispatchable なハンドルは `*mut c_void` を包んだ型で、構造体も生ポインタを持つため、どちらも `!Send` / `!Sync` でした。`vulkan::Device` を別スレッドへ渡せず、`Arc<Device>` も共有できません。

- 確かめ方: `fn assert_send<T: Send>()` に渡してコンパイルエラーになることを確かめた。
- 対応: ハンドルと、ポインタを持つ構造体に `unsafe impl Send` / `Sync` を付けた。ハンドルは不透明な名前であり、構造体の中のポインタは `unsafe` な command の中でしか読まれない。

### 照合の仕組み（新設）

上の誤りは、どれも「C のヘッダと比べれば分かる」ものでした。`crates/vulkan/tests/layout.rs` を追加し、generator が出した C のプログラムを Khronos のヘッダでコンパイルして、次を Rust 側と比べます。

- 構造体と共用体の大きさ、アラインメント、各 member の offset（1466 個）
- ビットフィールドの位置と幅（348 個）
- 定数、列挙値、フラグの値（6443 個）

プラットフォーム固有の型（`VK_USE_PLATFORM_*`）は、そのプラットフォームのヘッダが要るため対象外です。関数ポインタ型の引数の型は照合していません（A2 の種類の誤りは、型の対応付けを 1 か所にまとめ、単体テストで押さえています）。

## B. 寿命による安全性

### ash の現行の設計との比較

ash は 0.38 で builder 型と `build()` を廃止し、構造体自体が寿命パラメータを持ち、setter が `&'a` の参照やスライスを受け取る形になりました。旧 vulkan-rs も同じ形で、`build()` はありませんでした。基本の経路（スライス、単一の参照、`CStr`、`push_next`）は、一時値への参照が残るとコンパイルエラーになっていました。

そのうえで、次の抜け穴がありました。

### B1. `push_next` が、安全なコードから任意のポインタを辿っていた

旧 `push_next` は、渡された構造体の `p_next` を末尾まで辿り、そこに既存の chain をつないでいました。`p_next` は公開 field なので、`unsafe` を書かずに不正な値を入れられます。その値を安全な関数が参照外しするので、健全性の穴でした。

```rust
let mut features = vk::PhysicalDeviceVulkan12Features::default();
features.p_next = 0x1234 as *mut _;            // unsafe なしで書ける
let info = vk::DeviceCreateInfo::default().push_next(&mut features); // 旧: 0x1234 を読む
```

- 対応: 辿るのをやめた。渡された構造体の `p_next` は上書きする。`&mut T` を通した書き込みしか行わないので、どんな値が入っていても安全。

### B2. 型付きの生ポインタを受け取る安全な setter があった

`DeviceCreateInfo::p_queue_create_infos(*const DeviceQueueCreateInfo)` のように、寿命を追えない setter が、借用版の setter と並んで全 member に生えていました。field が公開されているので穴が増えるわけではありませんが、補完に出てくるので使ってしまいやすい状態でした。

- 対応: 型付きの生ポインタを受け取る setter は出さない。生ポインタを受け取る setter が残るのは、借用する型が存在しない `void*` とプラットフォームの不透明な型（`wl_display*` など）だけ。

### B3. ポインタのポインタで、内側の寿命を追えなかった

`enabled_extension_names(&'a [*const c_char])` は、配列の寿命は追いますが、各文字列の寿命は追いません。`CString` の `Vec` から `as_ptr()` を集めた後で `CString` を drop しても、コンパイルは通ります。ash も同じ形です。

- 対応: 文字列の配列は `&'a [vk::CStrPtr<'a>]` にした。`CStrPtr<'a>` は `*const c_char` と同じ layout で、`&'a CStr` からしか作れない（生ポインタからは `unsafe` で作る）。構造体へのポインタの配列（`ppGeometries` など）は `&'a [&'a T]` にした。

### B4. 長さを共有する複数の配列で、長さの食い違いを検出しなかった

`SubmitInfo` の `p_wait_semaphores` と `p_wait_dst_stage_mask` は `wait_semaphore_count` を共有します。setter はどちらも count を上書きするので、長さが違うと、後から設定した方の長さで短い方の配列を読み越えます。

- 対応: 同じ count を共有するほかの配列がすでに設定されているとき、長さが違えば panic する。command の wrapper でも同じ検査をする。

### B5. `user_data(&'a mut c_void)` のような意味のない setter があった

`void*` の member に対して `&'a mut c_void` を受け取る setter が出ていました。`c_void` への参照は作る方法がなく、使えません。

- 対応: `void*` は生ポインタを受け取る setter だけにした。

### B6. すべての構造体が寿命パラメータを持っていた

`vk::Extent2D<'a>` や `vk::Rect2D<'a>` にも寿命と `_marker` があり、構造体リテラルで書けず、自分の構造体の field に持つと `'static` を書く必要がありました。穴ではありませんが、寿命が「ポインタを持つ」という情報を運ばなくなっていました。

- 対応: ポインタを持ち得る構造体（1551 個のうち 1318 個）だけが寿命を持つ。

### B7. `Entry` を先に drop すると、関数ポインタが無効になった

`Entry` だけが読み込んだ loader を持っていて、`Instance` と `Device` は持っていませんでした。

- 対応: `Instance` と `Device` も loader への参照（`Arc`）を持つ。

### 意図して残した穴

次は、型では防いでいません。`unsafe` な command を呼ぶ側の責任です。trybuild のテストにもしていません。

- **公開 field への直接の代入**。結果を読むため、また setter がない形のために field は公開している。ポインタの field に直接書けば、寿命は追われない。
- **`void*` とプラットフォームの不透明な型**。`pUserData` や `wl_display*`。借用する Rust の型がない。
- **Vulkan が書き込んで返すポインタ**。`DisplayPropertiesKHR::display_name` など。生ポインタのまま返すので、読むには `unsafe` が要る。
- **構造体が `Copy` であること**。コピーは同じ寿命に縛られるので、寿命を越えることはない。
- **ハンドルの寿命**。`vk::Buffer` などは整数で、破棄後に使ってもコンパイルは通る。これを型で追うのは、この crate の層の役割ではない。

### テスト

`crates/vulkan/tests/ui/fail/` に、抜け穴 1 つにつき 1 ファイルの compile-fail テストを置きました（20 個）。`ui/pass/live_borrows.rs` は、同じ形で寿命が足りている場合に通ることを確かめます。

| ファイル | 確かめること |
| --- | --- |
| `slice.rs`、`byte_data.rs`、`shader_code.rs` | スライスが構造体より先に消えるとエラー |
| `single_reference.rs` | 単一の構造体へのポインタ |
| `nested_struct.rs` | 入れ子の構造体の中の借用が、外側に伝わる |
| `c_string.rs` | `CStr` |
| `string_array.rs`、`string_array_elements.rs` | 文字列の配列（外側と内側） |
| `pointer_array_elements.rs` | 構造体へのポインタの配列の内側 |
| `union_pointer.rs` | ポインタを選んだ共用体 |
| `p_next_dropped.rs`、`p_next_temporary.rs` | chain に入れた構造体が先に消える |
| `p_next_twice.rs` | 同じ構造体を 2 回 chain に入れる |
| `p_next_member_borrow.rs` | chain に入れた構造体が指す先が、root より先に消える |
| `p_next_output_read_while_chained.rs` | 出力用の chain に入れた構造体を、root が生きている間に読む |
| `output_slice_read_while_borrowed.rs` | 出力用の配列を、構造体が生きている間に読む |
| `descriptor_heap_source.rs`、`descriptor_heap_destination.rs` | descriptor heap への書き込み元（共用体と 2 段の構造体）と、書き込み先のメモリ |
| `no_lifetime_erasure.rs`、`returned_from_function.rs` | 寿命を消して持ち出す方法がない |

chain に入れられない構造体を `push_next` に渡す場合は、エラーメッセージが registry の内容に依存して変わるため、`lib.rs` の doctest（`compile_fail,E0277`）で確かめています。

## C. 高レベルの補助

- **二度呼びの列挙**: 1 回目で数を聞き、2 回目で受け取る形でした。`VK_INCOMPLETE` のときの再試行は動いていましたが、配列を伸ばす処理が分かりにくかったので、「数を聞き直してやり直す」形に書き直しました。
- **`VkResult` の成功コード**: 負の値をエラーとし、成功コードが複数ある command は `Ok` の中に `VkResult` を返す方針でした。これは残しました。`vkAcquireNextImageKHR`（`Option<(u32, bool)>`）、`vkQueuePresentKHR`（`bool`）、pipeline の作成（途中まで作られたハンドルを `Err` に含める）の特例も残しました。
- **void を返す列挙が `Result` を返していた**: `get_physical_device_queue_family_properties` が `Result<Vec<_>>` でした。`Vec<_>` にしました。
- **wrapper が出ない command があった**: 引数に `T**` などがあると、高レベルの wrapper を出さず `_raw` だけにしていました。借用の形にできない引数は生ポインタのまま通し、すべての command に wrapper を出すようにしました。あわせて、列挙の補助だった `_count` / `_into` はやめました。chain を付けた要素で列挙したいときは `_raw` を使います。
- **`unsafe`**: すべての command が `unsafe fn` です。Vulkan の valid usage は型で表せないので、この方針は残しました。Safety の説明はありませんでした（`clippy::missing_safety_doc` を抑止）。各 command に、対応する仕様のページへのリンクと Safety の節を付けました。
- **`Debug`**: 列挙型は数値でしか表示されず、構造体には `Debug` がありませんでした。列挙型とフラグは名前で表示し、構造体は `debug` feature で `Debug` を derive します。
- **フラグの演算**: `|`、`|=`、`&` しかありませんでした。`&=`、`^`、`^=`、`!`、`intersects`、`const` で使える `union` を足しました。

## D. `vulkan-alloc`

TLSF で `VkDeviceMemory` を分割する allocator です。単体テストが 38 個あり、分割と解放の処理はよく検証されていました。設計は残し、次を直しました。

- **allocate flags が違う確保が、同じ block を共有していた**。`memory_allocate_flags` は新しい block を作るときにしか使われず、既存の block を再利用するときは見ていませんでした。`DEVICE_ADDRESS` を付けずに作った block に、`DEVICE_ADDRESS` が要る buffer が入り得ました。block ごとに flags を持ち、一致するときだけ共有するようにしました。
- **export できるメモリを確保できなかった**。`VkExportMemoryAllocateInfo` を渡す方法がありませんでした。`AllocationCreateDesc::export_handle_types` を足しました。export するメモリは、分割せず専用の `VkDeviceMemory` にします（export した fd は、メモリ全体を指すため）。
- **`Send` でなかった**。`Send` にしました。
- **確保のたびに関数ポインタ表（約 5 KB）をコピーしていた**。`Arc<Device>` を持つようにしました。

dma-buf の import は allocator を通りません。import するメモリは、相手が決めた大きさと配置を持つ専用の `VkDeviceMemory` で、`vkAllocateMemory` を直接呼びます。allocator と同じ device で併用できます（`examples/dmabuf-roundtrip` が、export 側を allocator で、import 側を直接の確保で行っています）。

その後、hizukuri の方針（スクリプトの誤りで落ちない）に合わせて、次も変えました。

- **確保が残ったまま `Allocator` を drop すると panic していた**。`log` crate に報告して続けるようにしました。明示的に全部を解放する `free_all` と、残っている確保の一覧 `live_allocations` を足しました。
- `map` が返す guard が drop で unmap することが、文書から分からなかった（利用側が `unmap_memory` と二重に呼んだ）。crate の文書と README に書きました。

残っている性質:

- 内部は `&mut self` で、スレッド間で共有するには利用側で `Mutex` に入れます。
- 破棄の順序（資源、確保、allocator、device）は、型では保証しません（[design.md](design.md) の `vulkan-alloc` の節）。

## E. codegen の構造

- `main.rs` 1 ファイルに 3480 行（うちテストが約 870 行）。文字列の連結と、member 名や command 名での条件分岐が並んでいました。
- registry の解釈は、Python のスクリプトが Khronos の `VulkanObject` を使って行い、JSON にして Rust へ渡していました。**この分担は残しました**。`vk.xml` の解釈（alias、拡張による列挙値の追加、`api` の絞り込みなど）を Khronos 自身のコードに任せられるためです。
- Rust の名前（`rust_name`）を Python 側でも作っていて、同じ命名規則が Python と Rust の両方にありました。Python は C の名前だけを出し、命名は Rust 側だけで行うようにしました。
- テストは、生成結果に特定の文字列が含まれるかを見るものと、`cargo test` の中から `cargo check` を呼ぶものでした。後者は、examples を workspace に入れることで不要にしました。
- Vulkan-Headers を更新したときの差分は、`vulkan-ir.json`（registry の差分）と生成コード（API の差分）の両方に出ます。これは追いやすいので残しました。

書き直した後の構成は [design.md](design.md) にあります。

## F. ビルド時間と大きさ

`vulkan` crate 単体、`--features std,wayland`、24 スレッドの環境での値です。

| | 旧 | 新（`debug` あり） | 新（`debug` なし） |
| --- | --- | --- | --- |
| debug ビルド | 4.7 秒 | 6.2 秒 | 5.5 秒 |
| release ビルド | 4.6 秒 | 5.5 秒 | 4.8 秒 |
| `libvulkan.rlib`（debug） | 51 MB | 60 MB | 57 MB |
| `libvulkan.rlib`（release） | 26 MB | 39 MB | 33 MB |

増えたのは、`Debug` の実装、alias の定数、フラグの演算子、Safety の文書です。ビルド時間は問題になる大きさではありません。`rlib` は中間生成物で、最終的なバイナリには使った関数しか入りません。

関数ポインタ表は、`DeviceFn` が約 5 KB、`InstanceFn` が約 0.7 KB です。device を作るときに、すべての device command（約 700 個）を `vkGetDeviceProcAddr` で読みます。

## G. 文書、テスト、CI

- 文書は README の版の規則と更新手順だけでした。`AGENTS.md`、設計の文書、crate の文書を足しました。
- CI はありませんでした。`.github/workflows/ci.yml` を足しました。初回の実行で、submodule の tag がない、`libwayland-dev` と `libvulkan-dev` がない、Rust の版がローカルと違う、の 3 点で失敗し、直しました。
- 実際の Vulkan 実装を呼ぶテストはありませんでした（examples を手で動かすだけ）。lavapipe でも動く結合テストを足しました。
- リポジトリ直下に example が出力した `*.ppm` が残る作りでした。出力先を `target/example-output/` に変えました。

## 確かめたこと、確かめていないこと

実行して確かめたこと:

- `cargo fmt --check`、`cargo clippy --workspace --all-features --all-targets`、`cargo test --workspace --all-features` が通る。
- C のヘッダとの照合が、SDK 1.4.350.1、1.4.357.0、1.4.363.0 の 3 つの版で通る。
- 結合テスト（`crates/vulkan/tests/device.rs` の 11 個、`crates/vulkan-alloc/tests/device.rs` の 6 個）が、RADV と lavapipe で通る。validation layer（1.4.350）を有効にして、エラーが出ない。descriptor heap での描画を含む。
- GitHub Actions の CI（Ubuntu 24.04、Mesa 25.2 の lavapipe、validation layer 1.3.275）が通る。
- examples 8 個が RADV で動く。headless の 6 個は lavapipe でも動く。出力画像を目で確認した。validation layer のメッセージは出ない。
- Wayland の 2 個が niri の上で動く（swapchain の作成、描画、present、リサイズなしで 120 フレーム）。
- `VK_EXT_surface_maintenance1` による present mode の互換性の照会が、RADV と niri の組み合わせで結果を返す。

確かめていないこと:

- **ティアリング（`IMMEDIATE` / `FIFO_RELAXED`）と HDR（`HDR10_ST2084`、`VK_EXT_hdr_metadata`）での present**。niri の surface が `MAILBOX` と `FIFO`、`SRGB_NONLINEAR` と `PASS_THROUGH` しか提示しないため、選べなかった。コードは書いてあり、型と validation は通るが、実際の表示は見ていない。
- **`VK_KHR_display` / `VK_EXT_acquire_drm_display` で画面を取得する経路**。DRM master が要る。Wayland session の中からは、display の列挙が 0 個を返すことだけ確かめた。
- **CI の上での descriptor heap**。CI の lavapipe（Mesa 25.2）は `VK_EXT_descriptor_heap` を持たないので、そのテストは CI では skip される。ローカルの RADV と lavapipe（Mesa 26.2）では通る。
- **Linux x86_64 以外**。照合テストは実行した環境の ABI を見る。Windows、macOS、aarch64 では動かしていない。
- **プラットフォーム固有の構造体の layout**（`VK_USE_PLATFORM_*`）。照合の対象外。
- **Vulkan Video**。構造体の layout は照合したが、実際の decode / encode は動かしていない。

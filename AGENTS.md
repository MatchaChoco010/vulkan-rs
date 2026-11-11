# vulkan-rs

Khronos の Vulkan-Headers から生成する、pure Rust の Vulkan バインディングです。hizukuri（ゲームエンジン兼 Wayland compositor）が ash の代わりに使います。このリポジトリのコードは、エージェントが責任を持って管理します。

- 何が問題で、なぜ作り直したか: [docs/audit.md](docs/audit.md)
- 構成、API の規則、更新の手順: [docs/design.md](docs/design.md)
- スクリプトの誤りで落ちない層についての知見: [docs/safety-layer.md](docs/safety-layer.md)

hizukuri は Vulkan 1.4 と `VK_EXT_descriptor_heap` を前提にします。新しく書く例やテストは、descriptor set ではなく descriptor heap を使ってください。

## 構成

| 場所 | 内容 | 編集 |
| --- | --- | --- |
| `crates/vulkan/src/lib.rs`、`src/vk/mod.rs` | `Entry` / `Instance` / `Device`、プラットフォームの型、trait | 手で書く |
| `crates/vulkan/src/vk/format.rs`、`extension.rs` | `vk::Format::info` と `vk::ExtensionInfo` の型。表は生成物 | 手で書く |
| `crates/vulkan/src/vk/generated/` | バインディング | **生成物。手で編集しない** |
| `crates/vulkan/tests/generated/` | C のヘッダとの照合に使う表 | **生成物。手で編集しない** |
| `crates/vulkan/tests/` | API の形、寿命、照合、実機の結合テスト | 手で書く |
| `crates/vulkan-registry/vulkan-ir.json` | registry の dump | **生成物。手で編集しない** |
| `crates/vulkan-codegen/` | generator | 手で書く |
| `crates/vulkan-alloc/` | `VkDeviceMemory` の suballocator | 手で書く |
| `examples/` | 使い方の例。`src/shaders/*.spv` は `tools/compile-shaders` の出力 | 手で書く |
| `crates/vulkan/tests/shaders/` | 結合テストの shader。`*.spv` は `tools/compile-shaders` の出力 | 手で書く |
| `rust-toolchain.toml` | Rust の版の固定 | 上げるときは下の手順 |
| `third_party/vulkan/` | Vulkan-Headers の submodule。SDK タグに固定 | `tools/update-vulkan-headers` で動かす |

## 生成の手順

clone した直後は submodule が空です。

```sh
git submodule update --init
```

generator を変えたら、再生成して、生成物も一緒に commit します。

```sh
./tools/generate-vulkan           # 再生成（Python 3、cargo、rustfmt が要る）
./tools/generate-vulkan --check   # 再生成して、commit 済みの内容と違えば失敗
```

Vulkan-Headers を新しい SDK タグへ上げるとき。

```sh
./tools/update-vulkan-headers     # submodule を進め、版を上げ、再生成する
```

example やテストの `.slang` を変えたとき（`slangc` が要る）。

```sh
./tools/compile-shaders
```

## 検証の手順

変更を commit する前に、次をすべて通します。

```sh
cargo fmt --check
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo test --workspace --all-features
cargo test -p vulkan --no-default-features
./tools/generate-vulkan --check      # commit した後で。生成物が commit 済みの内容と同じか
./tools/compile-shaders --check
```

1 つ失敗したら、そこで止めて直します。`a && b; c` のように書くと、`a` が失敗しても `c` が実行されます。失敗したまま commit や push をしないよう、検査と commit は別の command にしてください。

`debug` feature を外すと、構造体に `Debug` がありません。テストで構造体を比べるときは、`assert_eq!` ではなく `assert!(a == b)` を使います（`--no-default-features` のテストがコンパイルできなくなります）。

`cargo test` のうち次の 2 つは、環境が足りないと skip します。skip したら、そのことを報告に書いてください。

- `layout`: C のヘッダとの照合。submodule と C コンパイラ（`cc`）が要る。`VULKAN_RS_REQUIRE_LAYOUT_TEST=1` で、skip せず失敗する。
- `device`: 実際の Vulkan 実装での結合テスト。Vulkan 1.3 の device が要る。`VULKAN_RS_REQUIRE_DEVICE_TEST=1` で、skip せず失敗する。

lavapipe（ソフトウェア実装）で動かすとき。

```sh
VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.x86_64.json cargo test --workspace --all-features
```

`VK_DRIVER_FILES` を空文字にすると、loader がドライバを 1 つも見つけられず、`ERROR_INCOMPATIBLE_DRIVER` になります。実機で動かすときは、変数を設定しないでください。

API を変えたら、examples を実際に動かします。headless のものは画像を `target/example-output/` に書きます。

```sh
cargo run -p vulkan-headless-triangle
cargo run -p vulkan-dmabuf-roundtrip
VULKAN_EXAMPLE_FRAMES=120 cargo run -p vulkan-wayland-dynamic-rendering   # Wayland session が要る
```

寿命の compile-fail テスト（`crates/vulkan/tests/ui/fail/*.stderr`）は、rustc のエラーメッセージと照合します。メッセージは rustc の版で変わるので、`rust-toolchain.toml` で版を固定しています。CI も同じ版を使います。

Rust の版を上げるとき。

1. `rust-toolchain.toml` の `channel` を書き換える。
2. `./tools/generate-vulkan` をやり直す（rustfmt の出力が変わることがある）。
3. `TRYBUILD=overwrite cargo test -p vulkan --test lifetime` で期待値を作り直し、差分が「メッセージの文言の変化」だけであることを確かめる。
4. 上の検証を全部通す。

## 守ること

- **生成物を手で編集しない**。直すのは generator（`crates/vulkan-codegen`）です。`generated/` と `vulkan-ir.json` の差分は、必ず `tools/generate-vulkan` の出力にします。
- **C のヘッダと合わない出力を、テストを緩めて通さない**。`layout` テストが失敗したら、generator の誤りです。
- **寿命の規則を緩めない**。型付きの生ポインタを受け取る setter や、寿命を消す `build()` のようなものを足さない。setter の形を変えたら、`tests/ui/fail/` に、その形で寿命が足りない場合のテストを足します。
- **generator に、特定の構造体や command の名前での分岐を増やさない**。registry の属性から決められるなら、属性で決めます。決められない場合だけ、理由を comment に書いて個別に扱います（`emit::commands::special_wrapper` が例です）。
- **registry の dump（`tools/generate-vulkan`）には、generator が使う属性だけを写す**。使わない属性を写すと、Vulkan-Headers の更新で無関係に壊れます。
- **Vulkan-Headers は SDK タグ（`vulkan-sdk-*`）に固定する**。workspace の版の `+` の後ろは、その SDK タグと一致させます（`tests/api.rs` が確かめます）。
- **ash など他のプロジェクトのコードを写さない**。設計を参考にするのは構いません。
- **examples の出力をリポジトリに置かない**。出力先は `target/example-output/` です。
- `docs/` と `AGENTS.md` は日本語で書きます。技術用語と識別子は原語のままにします。`README.md`（ルートと各 crate）、コードの中の comment、rustdoc、commit message、PR の説明は英語です。公開リポジトリなので、README は英語を正とし、日本語の文書へのリンクを置きます。
- **`vulkan-alloc` と手書きの層に、panic を足さない**。上に載る層は、スクリプトの誤りで落ちないことを求めます。入力の誤りは `Result` か `Option` で返します。意図して panic する条件は [docs/safety-layer.md](docs/safety-layer.md) の 5 節に一覧があります。増やしたら、そこに足します。
- 公開 API の方向を大きく変えるとき（寿命の扱い、command の wrapper の形、`Entry` / `Instance` / `Device` の役割）は、先にユーザーに確認します。

## 環境についての注意

- examples と結合テストは、validation layer（`VK_LAYER_KHRONOS_validation`）があれば有効にします。古い example（`headless-triangle` など）は、validation layer がないと起動しません。
- `sudo`、パッケージの導入、ホストの設定の変更が必要になったら、作業を止めて確認します。
- `VK_EXT_descriptor_heap` は新しい拡張です。Mesa 26 以降の RADV と lavapipe で動きます。CI（Ubuntu 24.04）の lavapipe は古く、descriptor heap、DRM format modifier、fence と semaphore の fd のテストは skip されます。何が skip されたかは、CI の job の summary に出ます。
- validation layer は、結合テストの答え合わせです。エラーが出たら、まずテストや example の側の誤りを疑ってください。
- CI は、submodule の tag を別に取得します（`tools/generate-vulkan` が SDK タグの名前を必要とするため）。`--all-features` は `linked` feature を含むので、link に `libvulkan.so`（`libvulkan-dev`）が要ります。

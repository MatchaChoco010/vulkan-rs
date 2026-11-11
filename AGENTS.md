# vulkan-rs

Khronos の Vulkan-Headers から生成する、pure Rust の Vulkan バインディングです。hizukuri（ゲームエンジン兼 Wayland compositor）が ash の代わりに使います。このリポジトリのコードは、エージェントが責任を持って管理します。

- 何が問題で、なぜ作り直したか: [docs/audit.md](docs/audit.md)
- 構成、API の規則、更新の手順: [docs/design.md](docs/design.md)

## 構成

| 場所 | 内容 | 編集 |
| --- | --- | --- |
| `crates/vulkan/src/lib.rs`、`src/vk/mod.rs` | `Entry` / `Instance` / `Device`、プラットフォームの型、trait | 手で書く |
| `crates/vulkan/src/vk/generated/` | バインディング | **生成物。手で編集しない** |
| `crates/vulkan/tests/generated/` | C のヘッダとの照合に使う表 | **生成物。手で編集しない** |
| `crates/vulkan/tests/` | API の形、寿命、照合、実機の結合テスト | 手で書く |
| `crates/vulkan-registry/vulkan-ir.json` | registry の dump | **生成物。手で編集しない** |
| `crates/vulkan-codegen/` | generator | 手で書く |
| `crates/vulkan-alloc/` | `VkDeviceMemory` の suballocator | 手で書く |
| `examples/` | 使い方の例。`src/shaders/*.spv` は `tools/compile-shaders` の出力 | 手で書く |
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

example の `.slang` を変えたとき（`slangc` が要る）。

```sh
./tools/compile-shaders
```

## 検証の手順

変更を commit する前に、次をすべて通します。

```sh
cargo fmt --check
cargo clippy --workspace --all-features --all-targets
cargo test --workspace --all-features
./tools/generate-vulkan --check
```

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

寿命の compile-fail テスト（`crates/vulkan/tests/ui/fail/*.stderr`）は、rustc のエラーメッセージと照合します。toolchain を上げてメッセージが変わったら、内容を確かめたうえで更新します。

```sh
TRYBUILD=overwrite cargo test -p vulkan --test lifetime
```

## 守ること

- **生成物を手で編集しない**。直すのは generator（`crates/vulkan-codegen`）です。`generated/` と `vulkan-ir.json` の差分は、必ず `tools/generate-vulkan` の出力にします。
- **C のヘッダと合わない出力を、テストを緩めて通さない**。`layout` テストが失敗したら、generator の誤りです。
- **寿命の規則を緩めない**。型付きの生ポインタを受け取る setter や、寿命を消す `build()` のようなものを足さない。setter の形を変えたら、`tests/ui/fail/` に、その形で寿命が足りない場合のテストを足します。
- **generator に、特定の構造体や command の名前での分岐を増やさない**。registry の属性から決められるなら、属性で決めます。決められない場合だけ、理由を comment に書いて個別に扱います（`emit::commands::special_wrapper` が例です）。
- **registry の dump（`tools/generate-vulkan`）には、generator が使う属性だけを写す**。使わない属性を写すと、Vulkan-Headers の更新で無関係に壊れます。
- **Vulkan-Headers は SDK タグ（`vulkan-sdk-*`）に固定する**。workspace の版の `+` の後ろは、その SDK タグと一致させます（`tests/api.rs` が確かめます）。
- **ash など他のプロジェクトのコードを写さない**。設計を参考にするのは構いません。
- **examples の出力をリポジトリに置かない**。出力先は `target/example-output/` です。
- 文書は日本語で書きます。技術用語と識別子は原語のままにします。コードの中の comment と rustdoc は英語です。commit message は英語の命令形です。
- 公開 API の方向を大きく変えるとき（寿命の扱い、command の wrapper の形、`Entry` / `Instance` / `Device` の役割）は、先にユーザーに確認します。

## 環境についての注意

- examples と結合テストは、validation layer（`VK_LAYER_KHRONOS_validation`）があれば有効にします。古い example（`headless-triangle` など）は、validation layer がないと起動しません。
- `sudo`、パッケージの導入、ホストの設定の変更が必要になったら、作業を止めて確認します。
- GitHub Actions の workflow（`.github/workflows/`）は、このリポジトリで push するまで一度も動いていません。初回の実行で失敗したら、workflow を直してください。

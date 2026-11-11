# 誤った使い方で落ちない層についての知見

hizukuri の論点 GFX-1（スクリプトに見せる GPU の API の抽象度と安全）の材料です。バインディングを作った立場から、次の 3 つをまとめます。

1. Vulkan の誤った使い方を、型で防げるもの、実行時の検査で防げるもの、検査では防げないものに分ける。
2. registry が持つ情報のうち、検査の生成に使えるものと使えないもの。Khronos の validation layer との関係。
3. 検査の層を、vulkan-rs に置くか、hizukuri に置くか。

検査の層そのものは、この文書では実装していません。hizukuri の gfx-spike が `spikes/gfx-script/` で検証する内容と重ならないよう、ここでは「バインディングと registry から何が言えるか」に絞ります。数値は、断りがない限り `vulkan-sdk-1.4.363.0` の registry と、このリポジトリを開発したホスト（Radeon RX 9060 XT、RADV、Mesa 26.2.6、validation layer 1.4.350）での実測です。

## 1. 前提

Vulkan の仕様は、valid usage を守らない呼び出しの結果を定めていません。ドライバは検査をしないので、誤った呼び出しは、その場で落ちることも、何も起きないことも、後で別の場所で壊れることもあります。「落ちる」と言うとき、次の 4 つを分けて考える必要があります。

| 種類 | 例 | 影響 |
| --- | --- | --- |
| プロセスが落ちる | ドライバの中での不正なメモリアクセス、loader の abort | compositor なら、デスクトップ全体が止まる |
| GPU が応答しなくなる | 終わらない shader、device lost | 全ての command が失敗する。復旧には device の作り直しが要る |
| 結果が壊れる | 画面の乱れ、古い内容の表示 | 落ちないが、誤りに気づきにくい |
| ホストの panic | バインディングや検査の層自身の `assert` | Rust の側の問題。捕まえられるが、状態が中途半端になる |

もう 1 つの前提は、**バインディングの型が守るのは、Rust で書くホストの誤りであって、スクリプトの誤りではない**ことです。スクリプトから来る値は、実行時に初めて分かります。寿命つきの構造体は、ホストが `vk::BufferCreateInfo` を組み立てるときの誤りを防ぎますが、スクリプトが「破棄した buffer の ID」を渡してくることは防ぎません。2 節の分類は、この区別の上にあります。

## 2. 誤った使い方の分類

### 2.1 型で防げるもの

vulkan-rs がすでに防いでいるものです。ホストのコードが対象です。

| 誤り | 仕組み |
| --- | --- |
| 構造体が、指す先より長生きする | 寿命パラメータ。compile-fail テスト 20 個（`crates/vulkan/tests/ui/fail/`） |
| 配列の長さと count の不一致 | setter と wrapper がスライスを受け取る |
| `sType` の誤り | `Default` が設定する |
| `pNext` に付けられない構造体を付ける | `Extends<Root>` |
| 列挙型やフラグの型の取り違え | 型ごとの newtype |
| `VkBool32` に 0 と 1 以外を入れる | setter と wrapper が `bool` を受け取る |
| 配列の引数を値で渡す、構造体の layout の誤り | C のヘッダとの照合テスト |

型で表せるが、vulkan-rs が意図して表していないものです。C の API と 1 対 1 という方針から外れるためです。上に載せる層が受け持ちます。

| 誤り | 型で表すなら |
| --- | --- |
| 破棄したハンドルを使う | 所有権を持つ型（RAII）。ハンドルは `Copy` な整数なので、今は追えない |
| 別の device のハンドルを渡す | device に結びついた型 |
| 外部同期の違反 | `&mut` を要求する。`externsync` から生成できる（3 節） |
| command buffer の状態の誤り（記録中でないのに記録する、など） | typestate |
| null のハンドルを必須の引数に渡す | `NonNull` 相当の型 |
| 列挙型に未定義の値を入れる | `from_raw` を持たない閉じた `enum`。ただし、ドライバが返す未知の値を受け取れなくなる |

### 2.2 実行時の検査で防げるもの

スクリプトの誤りを対象にする検査です。必要な状態と、費用の見積もりを付けます。費用の根拠は 4 節です。

| 検査 | 持つ状態 | 費用の見積もり |
| --- | --- | --- |
| **ハンドルの有効性、種類、所有者**。破棄済み、二重破棄、型の違い | 世代つきの番号の表。スクリプトには番号だけを渡す | 参照 1 回が約 1 ns。作成と破棄は表の更新 |
| **使用中の資源の破棄**。submit した command buffer が参照する資源を破棄する | submit ごとの参照の一覧、または世代番号。破棄を、GPU が使い終わるまで遅らせる | 参照の記録が command あたり数 ns。fence か timeline semaphore の確認が frame あたり 1 回 |
| **引数の値**。未定義の列挙値、未定義のフラグ、0 であってはならない値、範囲（`offset + size` が buffer の大きさ以内） | なし（定数の表と、資源の大きさ） | 比較が数回。数 ns |
| **format と用途の組み合わせ、device の limits** | `vkGetPhysicalDeviceFormatProperties2` と limits の結果を、起動時に取ってキャッシュ | 表引き。作成系の command だけなので無視できる |
| **拡張、version、feature が有効か** | 有効にした拡張と feature の集合 | 表引き。起動時か、pipeline の作成時 |
| **command buffer の状態**。記録中か、render の内か外か、begin と end の対応 | command buffer ごとの状態機械 | command あたり分岐が数回。数 ns |
| **draw に必要な状態が揃っているか**。pipeline、dynamic state、vertex buffer、push data | 記録中の「設定済み」の bit 集合 | draw あたり数 ns〜十数 ns |
| **image の layout と、queue family の所有** | image ごと（厳密には subresource ごと）の現在の layout | subresource を区別しないなら O(1)。区別するなら範囲の木が要り、数十 ns 以上 |
| **読み書きの競合（barrier の不足）** | 資源ごとの最後の読み書きの stage と access | validation layer の同期の検査に相当し、重い。検査するより、barrier をコアが挿入して「誤りを表現できない」形にする方が安い |
| **map したメモリの範囲外への書き込み** | 確保の大きさ | 比較 1 回。スクリプトにはポインタを渡さず、範囲つきのコピーだけを見せる |
| **SPIR-V と pipeline の整合**。SPIR-V として正しいか、使う capability に対応する feature が有効か、vertex 入力や attachment の format が合うか | SPIR-V の reflection の結果 | pipeline の作成時に 1 回。ミリ秒の単位 |
| **資源の量**。確保の総量、個数、image の大きさ、draw の数 | 予算のカウンタ | 加算と比較 |

この表から言えることは 2 つです。

- **1 回あたりの費用は小さい**。高いのは費用ではなく、状態を正しく保つための実装の量です。「Vulkan に近い高さ」をそのまま見せるなら、上の表のほぼ全部が要り、validation layer の中心部分を作り直すことになります。
- **検査する代わりに、誤りを表現できない API にできる項目がある**。barrier、image の layout、破棄の順序、command buffer の状態は、スクリプトに書かせなければ、検査が要りません。

### 2.3 検査では防げないもの

| 事象 | なぜ防げないか | 緩和の手段 |
| --- | --- | --- |
| **shader の無限ループ、極端に長い実行** | shader の停止性は判定できない。GPU の実行を途中で止める API はない | ドライバの watchdog に任せる（device lost になる）。shader をコアが生成または変換するなら、ループの回数に上限を入れられるが、任意の shader を許す方針とは両立しない |
| **shader の中の範囲外アクセス** | index は GPU の上で決まる | `robustBufferAccess` と `VK_KHR_robustness2`（buffer と image の範囲外を定義済みの動作にする）。descriptor heap については次節 |
| **device lost** | ドライバ、ハードウェア、ほかのプロセス、スリープからの復帰でも起きる。正しい使い方でも起きる | 検査ではなく、復旧の設計。全ての command が `ERROR_DEVICE_LOST` を返し得るものとして扱い、device と資源を作り直せるようにする。`VK_EXT_device_fault` で原因の手がかりを取る |
| **ドライバの不具合** | 仕様どおりの呼び出しで落ちる。新しい拡張ほど起きやすい | 使う機能を絞る。lavapipe での再現を手がかりにする。最後の手段は、GPU を触る処理を別プロセスに分けること |
| **文章でしか書かれていない valid usage** | 仕様の文を読んで検査を書くしかなく、網羅を機械的に確かめられない | validation layer を答え合わせに使う（3 節） |
| **メモリの枯渇による強制終了** | `ERROR_OUT_OF_DEVICE_MEMORY` が返る前に、OS がプロセスを止めることがある | 予算（2.2 の最後の行）で、そこまで行かせない |
| **検査の層自身の誤り** | 検査が漏れていれば、そのまま Vulkan に届く | fuzz と validation layer による答え合わせ（5 節） |

### 2.4 descriptor heap と検査

hizukuri は `VK_EXT_descriptor_heap` を前提にします。検査の観点では、良い面と悪い面があります。

良い面は、**CPU の側で検査する状態が大きく減る**ことです。descriptor set の仕組みでは、set layout、pipeline layout、pool、set の確保、更新、bind、pipeline layout の互換性、「この draw の時点で、必要な set が全部 bind されているか」を追う必要がありました。descriptor heap では、heap は buffer で、descriptor は `vkWriteResourceDescriptorsEXT` がホストのメモリに書く不透明なバイト列で、shader は番号で引きます。pipeline layout もありません。検査が要るのは次だけです。

- 書き込み先が heap の範囲内で、descriptor の大きさとアラインメントに合っているか。vulkan-rs の wrapper はスライスを受け取るので、範囲外には書けません。番号から offset への計算は、上の層の仕事です。
- driver が予約する範囲（`minResourceHeapReservedRange`）に書いていないか。
- heap を bind しているか、push data の大きさが `maxPushDataSize` 以内か。

悪い面は、**shader が使う番号を、CPU の側から検査できない**ことです。番号は push data や buffer の中のデータで、shader の中で計算もできます。未書き込みの slot や、型の違う descriptor を指した場合の動作は未定義です。防ぐ手段は検査ではなく、heap の中身を常に定義済みに保つことです。

- heap の全 slot を、作成時に、有効な descriptor（1×1 の image、小さい buffer など）で埋めておく。
- 資源を破棄するとき、その slot を先に dummy の descriptor に書き戻す。破棄した資源の番号を shader が使っても、「dummy を読む」という定義済みの動作になる。
- 型ごとに番号の範囲を分け、image の番号で buffer の slot を引けないようにする。型の違いを shader の側で防ぐ手段はないので、これは「番号の表を層が発行する」ことでしか保証できません。

`VK_KHR_robustness2` の `nullDescriptor` を descriptor heap と組み合わせたときの動作は、このリポジトリでは確かめていません。

## 3. registry が持つ情報

### 3.1 検査の生成に使えるもの

件数は `vk.xml` を数えた値です。

| 情報 | 件数 | 内容 | 使い道 | vulkan-rs での扱い |
| --- | --- | --- | --- | --- |
| `len` | member 366、引数 214 | 配列の長さを持つ member や引数 | 長さの検査 | スライスにして、型で解決済み |
| `optional` | member 2116、引数 421 | null や 0 を許すか | 必須のハンドル、ポインタ、フラグ、count が 0 でないことの検査 | 引数の `Option` に反映。値の検査はしていない |
| 列挙値とフラグの bit の一覧 | 列挙型 200、フラグ 179 | 定義済みの値 | 未定義の値の検出 | 定数としてある。「既知の値か」を返す関数は未生成 |
| 値や command を提供する拡張と version | 全ての command、構造体、列挙値 | どれを有効にすれば使えるか | 有効化していない機能の使用の検出 | command と構造体は doc comment に出している。表としては未生成 |
| 拡張の `depends`、昇格先 | 拡張 477 | 拡張どうしの依存 | 有効にする拡張の組の検査 | **`vk::ExtensionInfo` として生成済み** |
| `<format>` | format 297 | block の大きさ、component、plane | 大きさの計算、aspect の検査、plane ごとの処理 | **`vk::Format::info` として生成済み** |
| `structextends`、`allowduplicate` | 関係 1260 | `pNext` に付けられる相手、重複の可否 | chain の検査 | 付けられる相手は型で解決済み。重複は未検査 |
| `externsync` | command 353、引数 363 | 外部同期が要る引数 | どのハンドルを排他にするかの表 | 未使用。全て `&self` |
| ハンドルの `parent`、`objtypeenum` | ハンドル 59 | 親と `VkObjectType` | 「この device のものか」の検査、名前付け | `Handle::TYPE` を生成済み。親は dump しているが未使用 |
| `queues`、`renderpass`、`cmdbufferlevel`、`tasks` | command 約 270 | 記録できる queue の種類、render の内か外か、primary か secondary か | command buffer の状態機械の検査 | 未使用 |
| `successcodes`、`errorcodes` | 全ての command | 返り得るコード | 戻り値の形、どの command が `DEVICE_LOST` を返し得るか | 成功コードは wrapper の形に使用。エラーコードは dump しているが未使用 |
| `selector`、`selection` | member 10 | 共用体の、どの member が有効か | 共用体を tagged union として扱う | 未使用 |
| `<sync>` | stage 46、access 51 | stage と access の対応、queue の種類との対応 | barrier の stage と access の整合の検査、または barrier の自動生成 | 未使用 |
| `<spirvcapability>`、`<spirvextension>` | 202、114 | SPIR-V の capability を使える条件 | shader が宣言する capability と、有効な feature の照合 | 未使用 |
| `limittype` | member 981 | limits や properties の member が、上限か下限か bit 集合か | limits の表示、比較の向き | 未使用 |

「未使用」「未生成」のものは、どれも事実の表なので、generator に足せば出せます。優先度が高いと考える順は、(1) command ごとの要件（拡張、version、queue の種類、render の内外）、(2) 列挙値とフラグの「既知の値か」、(3) SPIR-V の capability の表、(4) `externsync` の表、です。

### 3.2 使えないもの、注意が要るもの

- **`validusage.json`**。valid usage の一覧で、27,415 件あります。うち 6,844 件は、上の表の属性から機械的に導かれるもの（implicit）で、残りの 20,571 件は仕様の文章（explicit）です。explicit は HTML の自然文なので、検査の生成には使えません。使い道は、エラーメッセージに VUID と文を添えることと、「どの VUID を自前で検査しているか」の棚卸しです。
- **`noautovalidity`**（member 364、引数 14）。これが付いた member は、`len` や `optional` から検査を生成してはいけません。別の member の値によって無視される、といった条件が文章で書かれています。`VkWriteDescriptorSet` の 3 つの配列がその例です。
- **`limittype`**。limits の member の種類は分かりますが、「どの command のどの引数がその limit に縛られるか」は書かれていません。それは explicit な VUID の文章にあります。
- **`externsync` の `maybe:` 形式**。`maybe:pDescriptorWrites[].dstSet` のように、条件つき、または構造体の奥の member を指すものがあります。機械的に扱えるのは `true` だけです。
- **式の `len`**。`codeSize / 4`、`(rasterizationSamples + 31) / 32` など。個別に扱うしかありません。

### 3.3 Khronos の validation layer との関係

validation layer は、次の検査を持っています。

- 引数の検査（null、列挙値、`sType`、`pNext`）。`vk.xml` から生成されていて、implicit な VUID に対応します。
- ハンドルの生存と親子の追跡。
- 状態を追う検査（core checks）。explicit な VUID を、人が C++ で書いたものです。
- スレッド安全の検査（`externsync` から生成）、同期の検査、GPU の上で動く検査（shader の中の index など）。

これを本番の防壁にすることは、勧めません。理由は 4 つです。

1. **重い**。4 節の実測で、command 1 つあたり 30〜80 倍です。
2. **止める層ではなく、報告する層である**。エラーを報告した後も、呼び出しはドライバに届きます。callback が `VK_TRUE` を返すと呼び出しを飛ばす仕組みはありますが、仕様は「アプリケーションは常に `VK_FALSE` を返すこと。`VK_TRUE` は layer の開発用」としています。
3. **全ての VUID を検査しているわけではない**。
4. **layer の版と、ドライバや拡張の版がずれる**。このリポジトリの CI（Ubuntu 24.04）の validation layer は 1.3.275 で、`VK_EXT_descriptor_heap` を知りません。

validation layer の正しい位置づけは、**検査の層の答え合わせ**です。「検査の層が通した呼び出しで、validation layer がエラーを出したら、検査の漏れ」として、テストで使います。vulkan-rs の結合テスト（`crates/vulkan/tests/device.rs`）は、validation layer のエラーが 1 件でも出たら失敗します。実際に、この作り直しの作業の中で、テストと example に書いた誤りを 4 件見つけました（dma-buf の plane layout の `size` と `arrayPitch`、`VK_EXT_descriptor_heap` が Vulkan 1.3 では `VK_KHR_maintenance5` に依存すること、shader が `shaderDrawParameters` を要求すること）。

## 4. 費用の実測

リポジトリには入れていない一時的なベンチマークで、release ビルド、RADV、20 万回の平均です。

| 操作 | validation layer なし | あり | 倍率 |
| --- | --- | --- | --- |
| `cmd_set_viewport` | 5 ns | 290 ns | 約 55 倍 |
| `cmd_copy_buffer`（region 1 個） | 29 ns | 870 ns | 約 30 倍 |
| buffer の作成と破棄 | 11 ns | 890 ns | 約 80 倍 |

ハンドルの表の参照は、6 万 5 千個の slot に対する乱数での参照 5000 万回の平均です。

| 方式 | 1 回あたり |
| --- | --- |
| 配列の添字と世代番号の比較 | 約 1 ns |
| `HashMap<u64, _>` | 約 6 ns |

読み取れることは次のとおりです。

- vulkan-rs の wrapper を通した command の記録は、5〜30 ns です。検査の層が command あたり数 ns〜十数 ns を足しても、同じ桁に収まります。
- validation layer の費用（数百 ns）は、その 1〜2 桁上です。自前の検査は、validation layer より 1 桁以上軽くできる余地があります。対象を、スクリプトに見せる API が生み出せる誤りに絞れるからです。
- 測ったのは 1 台、3 種類の command だけです。draw や barrier、同期の検査の費用は測っていません。

## 5. 検査の層をどこに置くか

**意見: 表は vulkan-rs に、状態と方針は hizukuri に置く。**

### vulkan-rs に置くもの

registry から機械的に生成できて、方針を含まない「事実の表」と、それを引く関数です。

- すでにあるもの: `vk::Format::info`、`vk::ExtensionInfo`、`vk::Handle::TYPE`、列挙値とフラグの定数と名前。
- 足せるもの: 3.1 の「未生成」の表。

理由は 2 つです。

1. **Vulkan-Headers の更新に追従するものは、generator と同じ場所にあるべき**です。新しい format や拡張が増えたとき、表が自動で更新されます。hizukuri の側に手書きの表を持つと、更新のたびにずれます。
2. **C のヘッダとの照合が効く**範囲に置けます。表の値が registry と一致することを、生成時に確かめられます。

### hizukuri に置くもの

状態を持つ検査と、方針です。ハンドルの表、破棄の遅延、layout や command buffer の状態の追跡、予算、エラーをスクリプトの言語にどう返すか、どこまでの自由度を見せるか。

理由は 3 つです。

1. **API の抽象度（GFX-1）と切り離せない**。何を検査するかは、スクリプトに何を見せるかで決まります。barrier を見せないなら barrier の検査は要らず、command buffer を見せないなら状態機械も要りません。見せるものが決まる前に、汎用の検査の層を作ることはできません。
2. **汎用に作ると、別のものになる**。「どんな使い方でも安全な Vulkan」は wgpu の中心部分に相当し、それ自体が大きなプロジェクトです。hizukuri の要求（descriptor heap を前提にする、dma-buf を import する、低い層を見せる）は、汎用の安全な層が切り捨てている部分にあります。
3. **vulkan-rs の方針を保てる**。vulkan-rs は「C の API と 1 対 1」を保つことで、照合テストが成り立っています。状態を持つ層を同じ crate に入れると、何が生成物で何が設計判断かの境界が崩れます。

### 後から切り出せるもの

hizukuri の中で育った部品のうち、状態は持つが方針は持たないもの（image の layout の追跡、command buffer の状態機械など）は、形が固まった後で、vulkan-rs の workspace の別 crate に切り出せます。最初からそうしないのは、利用者が 1 つしかないうちに汎用にすると、設計を誤るからです。`vulkan-alloc` は、その形の例です。

### 2 つの層の間の約束

検査の層は「Vulkan に届く前に止める」ので、その下の vulkan-rs が panic しないことを当てにします。vulkan-rs が panic する条件は、次のとおりです。検査の層は、これを先に確かめる必要があります。

| 条件 | 先に確かめる方法 |
| --- | --- |
| 読み込まれていない command を呼ぶ（有効にしていない拡張の command など） | `device.fp().create_swapchain_khr.is_some()`。または、起動時に、使う command が全て読めていることを確かめる |
| count を共有する複数の配列の長さが違う（setter と wrapper） | 長さを比べてから渡す |
| 構造体の member が長さを決める入力の配列の長さが違う（`get_acceleration_structure_build_sizes_khr` の `max_primitive_counts` など） | 同上 |
| `Entry::from_static_fn` に null の `vkGetInstanceProcAddr` を渡す | 渡す前に確かめる |
| `vulkan_alloc::MappedAllocation::byte_range` の範囲外 | `try_byte_range` を使う |

これ以外に、vulkan-rs と `vulkan-alloc` が意図して panic する箇所はありません。`vulkan-alloc` の内部の不変条件の `expect` が 2 か所あり、これは allocator 自身の不具合のときだけ起きます。`Vec` の確保の失敗は、Rust の既定どおり abort します。`_raw` の method と `fp()` の関数ポインタ表は、長さの `assert` を含みません。

## 6. hizukuri への提案

バインディングの側から見て、GFX-1 の議論に持ち込みたい点です。

1. **スクリプトには、Vulkan のハンドルもポインタも渡さない**。渡すのは、コアが発行する世代つきの番号だけにする。これで、2.2 の最初の 2 行（有効性と、使用中の破棄）が、表の参照と遅延の仕組みだけで片づきます。費用は約 1 ns です。
2. **「検査する」より「表現できなくする」を先に検討する**。barrier、image の layout、queue family の所有、破棄の順序は、資源の使い方の宣言からコアが導けます。自由度として残す価値があるのは、shader、pipeline の状態、データの流れ、pass の構成です。
3. **descriptor heap は、番号の表として見せる**。heap の全 slot を定義済みに保つ（2.4）ことを、層の不変条件にします。
4. **device lost からの復旧を、最初から設計に入れる**。検査では防げないので、後から足すと全体に手が入ります。
5. **検査の層のテストに、validation layer と lavapipe を使う**。スクリプトの API に乱数で呼び出しを流し、「検査を通った呼び出しで validation layer がエラーを出さない」「プロセスが落ちない」を確かめます。lavapipe は CPU の上の実装なので、ドライバの中の不正なアクセスが、GPU の停止ではなく、普通の segfault として観測できます。
6. **registry の表が要るときは、vulkan-rs に足す**。3.1 の「未生成」のものは、要るものから generator に入れます。

## 7. 確かめていないこと

- `VK_KHR_robustness2` の `nullDescriptor` と descriptor heap を組み合わせたときの動作。
- shader が終わらないとき、RADV と lavapipe がそれぞれどう振る舞うか（device lost になるか、止まるか）。
- validation layer の、GPU の上で動く検査と同期の検査の費用。
- 4 節の実測は、1 台、3 種類の command だけです。
- 2.2 の「費用の見積もり」のうち、ハンドルの表の参照以外は、実装して測った値ではありません。処理の内容からの見積もりです。

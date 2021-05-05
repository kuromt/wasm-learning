

WASM
---

- javascriptとの違いは
  - WASMは他の言語からコンパイルされて生成できる
    - 現在はc, c++, Rust, goからコンパイルされる
    - WASMはアセンブリ言語の機能を持つので、やろうと思えばWASMを直接書くことも可能
        - これがWASMのA(Assembly)の語源？
  - WASMはjavascriptより高速であることを目指して作られている
    - JavaScriptはインタープリター言語であるのに対してWASMはコンパイルされた実行バイナリだから高速、と理解
        - 正解だった(6)
- WASMを実行するランタイムは？
  - ブラウザで実行できる、Node.jsで実行できると言われている
  - 「Node.jsで実行できる」という言葉だけ見ると,Node.jsはjavascriptとWASMを実行できることになる
  - 「javascriptからwasmを呼び出せる」(2)とあるので、呼び出し関係はNode.js -> javscript -> WASM？
- `.wasm` ファイルをモジュールと呼ぶ(4)
  - このモジュールにはjavascriptからアクセスできる1つ以上のexportされた関数がある（main, startなど)
  - WASMはimportして実行する関数の詳細を把握していないので、unsafeとしてみなされる
  - 関数が返せるのは数値型のみ(i32/i64, f32/f64). 数字の意味を深く知る必要がある

- WASMに触れる(9)
  - WebAssemblyにRustを使う理由が[解説](https://moshg.github.io/rustwasm-book-ja/introduction.html)されている
  - wasm_bindgenがJavascriptとRustの橋渡しをする
    - JavascriptのオブジェクトをRustの構造体にマッピングするなど

Rust
---

- M1のMacにRustの開発環境をいれる
    - インストール方法
        - homebrew
        - rustup
            - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- ツール
    - rustup
      - システムにあるrustを管理するツール
    - cargo
      - rustとビルドツール
    - rustc
      - rustのコンパイラ
        - cargoが内部で叩いている？
    - racer
      - rustのコード補完ツール
      - vscodeのrust extentionがデフォルトでracerを使うようになっていた
      - `cargo install racer` でインストール

- Rustを覚える(7),(8)
  - セミコロンがあると文、セミコロンがないと式として解釈される
  - 外部からrustを呼ぶチュートリアルでリリースビルドすると"can't find library `embed`"が表示される
    - src/main.rs ではなく src/lib.rsファイルの中にコードを書く必要がある
  - 所有権が移るとその変数を使うことはできなくなる
    - 関数の引数に変数をいれるとき、所有権を関数に渡したくなければ参照先のアドレスを渡すように作る = 借用
    - 借用しない場合はデータのコピーが発生する
      - ためしに2つの要素のHashMapをforでpringするコードを書いて借用する場合としない場合の実行時間を測定 => 明らかに借用した方が速い
        - `for i in hash_map`: 13-16μs
        - `for i in &hash_map`: 9-11μs
  - RustのStringはコレクションだが添字で要素にアクセスすることは許可していない
    - スライスを使った取り出しはできる
  - `&{integer}`と `&integer`に違いはある？
    - `{integer}`はコンパイラが整数だけどどの型なのか正確に理解できない時に使われると書かれていた[参考](https://users.rust-lang.org/t/what-is-the-type-integer/10137)
  - 標準出力のformatで`{:?}`だとDisplayが実装されていない変数をprintできるのはなぜ？
  - selfと&selfの使い分け
    - selfの参照だけを渡すときは&self
  - generic型にメソッドを実装するときimplで `impl<T>` でも `impl` でもコンパイルが通り結果も変わらない. なぜ`<T>` が必要になる?
    - コンパイラに`Point<T>`の`<T>`がジュネリック型であることを教えるためと説明にあるが、これをかかなくても`<T>`と書いているだけでジェネリックだと分かるのでは？
      -  `<>`はジェネリック型のための記号ではない. `Point<f32>` とすることができる.
        - これは`Point<T>`のTがf32のときにだけ有効なメソッドとして実装するときに使うもの.
    - Rustはジェネリック型でコードを書いてもコンパイラがその型が使われている箇所を見て具体的な型のコードを生成するため、実高速度に影響が出ないようになっている = 単相化
      - コンパイルにかかる時間は増える？
    - 実装しているトレイトを制限して引数を指定する方法を使うことができる
      - システムライブラリが持つトレイトをある程度覚えておかないとRustを実装するときにつらそう
        - PartialOld: 値を比較するトレイト
        - Copy: 値をコピーするトレイト. ヒープに値を保持する型で必要になる.
        - Display: 出力するときに使う.
        - ToString: Stringに変換するトレイト.  `imple <T: Display> ToString for T` で実装されている
    - ライフタイムを指定しないとコンパイルできない関数がある
      - 安全性を考えるとあった方がいいのは分かるが、なくても動く他の言語もあるので、個人的には記述が冗長に思える
        - ライフライム注釈を使うとジェネリックな型でもダングリング参照が存在しないことを保証することができるとのこと
        - GCとの違いは？ライフタイム注釈があれば参照がなくなったオブジェクトがないかチェックする必要がなくなるのは分かるがGCを受け入れている言語がある以上, Rustでここまで厳密にライフタイムをプログラマに記述させる理由はなに？
          - [所有権とライフタイム](https://doc.rust-jp.rs/rust-nomicon-ja/ownership.html) を読んで、ライフタイムが必要な理由は理解した
      - ライフタイムをかかなくても参照を使う関数を書いてコンパイルできるのは、Rustの歴史的になんども宣言されたライフタイムをコンパイラが自動解釈するように改善されてきたため = ライフタイム省略規則 
    - 10.3 で「ライフタイムは一種のジェネリック」とあるが、意味が理解できないので調べる
      - TODO
    - test結果を表示するためには`PartialEq`と`Debug`トレイトが実装されている必要がある
      - 標準ライブラリの型はほとんどで実装している. 自作のenumやstructのテストをする場合は自分で実装する必要がある
      - 独自型へのトレイトの実装は自分でやってもいいが、`#[derive(PartialEq, Debug)]`をつけるだけでコンイラが自動生成してくれる
    - test
      - 単体テストは同一ファイルの中に書く
      - 結合テストは`src`の隣に`tests` ディレクトリを作りそこにテストコードを書く
        - 結合テストで使うがテストしたくないコードは `tests`にサブディレクトリを作りそこで定義すればいい

- パッケージ, クレート, モジュールの違い
  - パッケージ: クレートをビルドするためのCargoの機能
  - クレート: 木構造をしたモジュール群
    - バイナリクレートとライブラリクレートがある
    - src/binの下に複数のバイナリクレートを置ける
  - モジュール: useで指定する対象
    - modで宣言する
- モジュールの指定
  - モジュールを使うためのモジュールを指定する方法は絶対パスと相対パスの二種類ある
  - 毎回パスを書くのが大変なときのために, パスをスコープに納めるuseがある
    - useを使うときは慣例として対象が別で定義されていることが分かるように、`xxx::yyy` として呼び出す形でuseを指定する
      - `yyy` だとローカルで定義されているのか別モジュールで定義されているのか見ただけで区別できない


トラブルシュート
---

- vscodeにRustのextentionを入れてcargoでプロジェクトをつくるとvscodeでエラー
  - `vscode rustup not available` と `Couldn't start client Rust Language Server`
    - vscodeをreloadしても解決しない
    - `"rust-client.rustupPath": "rustup"` を `"rust-client.rustupPath": "/Users/kuromt/.cargo/bin/rustup"` に修正
    - reloadで解決
- `cargo build`でビルドするとエラー
  ```
  bookers:hello_world kuromt$ cargo build
   Compiling hello_world v0.1.0 (/Users/kuromt/git/wasm-learning/hello_world)
  error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-arch" "arm64" "-L" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.10x1umxe2ihpk42f.rcgu.o" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.130ef8m9io394ian.rcgu.o" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.1zzs0u4r4rr6c8qx.rcgu.o" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.2thlrb4v5nonu3ei.rcgu.o" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.3hcllqf9ly8u7vun.rcgu.o" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.3vxyxllmng5qxye8.rcgu.o" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.4bdiqv3f1a8kbho9.rcgu.o" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.5018g9xi71pjttcd.rcgu.o" "-o" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps/hello_world.5edqdpt8dwu6eb48.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/kuromt/git/wasm-learning/hello_world/target/debug/deps" "-L" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libstd-e40bd6a3af835df4.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libpanic_unwind-68aa428439efc3d3.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libobject-e8883902f5c364b6.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libaddr2line-a950b2eed913eb15.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libgimli-9ce7597f6151ffc2.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/librustc_demangle-6a556d2584ddcc81.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libhashbrown-e62ba676fce5c547.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/librustc_std_workspace_alloc-509c1845891b6a82.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libunwind-082f7f4ff792426f.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libcfg_if-28ab4d9e8295eb15.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/liblibc-e9e7769a4bdcb2d3.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/liballoc-94664336047de1ce.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/librustc_std_workspace_core-8df6af481e225202.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libcore-214021ad04ab7aee.rlib" "/Users/kuromt/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libcompiler_builtins-9eb7dc999785b765.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
  = note: xcrun: error: invalid active developer path (/Library/Developer/CommandLineTools), missing xcrun at: /Library/Developer/CommandLineTools/usr/bin/xcrun
  ```
  - `xcode-select --install` を実行すると解決
- `cargo install racer` でエラー
  - racerにstable版がないことが原因
  ```
  error[E0554]: `#![feature]` may not be used on the stable release channel
    --> /Users/kuromt/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-1.6.1/src/lib.rs:76:37
    |
  76 | #![cfg_attr(feature = "may_dangle", feature(dropck_eyepatch))]
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0554`.
  error: could not compile `smallvec`

  To learn more, run the command again with --verbose.
  warning: build failed, waiting for other jobs to finish...
  error: failed to compile `racer v2.1.45`, intermediate artifacts can be found at `/var/folders/ff/70gp6zcx1lxby3ptt3lbxbs80000gn/T/cargo-installiW4Q4x`

  Caused by:
    build failed
  ```
  - stable版ではないものをインストールするときはcargoに `+nightly` オプションが必要
    - ただし、Apple Siliconではサポートされていない
      ```
      $ cargo +nightly install racer
      error: toolchain 'nightly-aarch64-apple-darwin' is not installed
      ```
  - あきらめてvscodeのrust-analyzerを使うことにした
    - reloadするとrust-analyzerをダウンロードするか聞かれるのでダウンロードを実行
    - これはRust Extentionと競合するらしいので、Rust Extentionを無効化
    - ときどきコード補完がきかなくなる. そのときはreload windowsを実行すると治る.

- wasm-packをインストールしようとすると `wasm-pack-init: no precompiled binaries available for CPU architecture: arm64`
  - このシェルがApple Siliconをサポートしていない？
  - `cargo install wasm-pack` でインストールした

- `wasm-pack build`を実行するとエラー
  - メッセージ
  ```
  Error: no prebuilt wasm-opt binaries are available for this platform: Unrecognized target!
  To disable `wasm-opt`, add `wasm-opt = false` to your package metadata in your `Cargo.toml`.
  ```
  - [rustwasm/wasm-pack#913](https://github.com/rustwasm/wasm-pack/issues/913)で報告されているが未解決
    - 二種類のワークアラウンドが紹介されている
      - Cargo.tomlでwasm-optを無効化する
      - brewでwasm-optをインストールする(`brew install binarygen`)
    - ワークアラウンドを採用するか判断するためにwasm-optが何かを調べる
      - [WebAssembly/binarygen](https://github.com/WebAssembly/binaryen)によると"Loads WebAssembly and runs Binaryen IR passes on it."とのこと
        - これがないと動かないのでは？
      - 他のサイトをいくつか見ると、コンパイルの最適化のツールと理解. なくてもよさそう.
    - brewでインストールするとcargoでインストールした他のパッケージを使うときに何が起こるかわからないので、ひとまず無効化してチュートリアルを進める
      - チュートリアルでwasm-optが必須になったらまた考える

- `wasm-game-of-life/pkg`の中で`npm link`を実行するとエラー
  - sudoで実行する必要がある
  - log
  ```
  $ sudo npm link
  Password:
  npm notice created a lockfile as package-lock.json. You should commit this file.
  npm WARN wasm-game-of-life@0.1.0 No repository field.
  npm WARN wasm-game-of-life@0.1.0 No license field.

  up to date in 0.72s
  found 0 vulnerabilities

  /usr/local/lib/node_modules/wasm-game-of-life -> /Users/kuromt/git/wasm-learning/wasm-game-of-life/pkg
  bookers:pkg kuromt$ pwd
  /Users/kuromt/git/wasm-learning/wasm-game-of-life/pkg
  ```

疑問
---

- WASMが扱えるのは4GBまでとあったが、それより大きなモジュールを扱う手段はない？
- WASMはjavascriptより高速とあるが、そのWASMを呼ぶのにjavascriptを書かないといけない設計になっているのはなぜ？
    - メインロジックはWASMで呼び出しだけjavascriptなのはわかる
    - ただ、javascript側もwasmが何の関数をもち、その関数の返り値が何を意味するかを把握した人しか書けない
        - pydocのようなwasmのdocが必要になる？

- WASMの環境構築の[セットアップ](https://moshg.github.io/rustwasm-book-ja/game-of-life/setup.html)でwasm-packはcurlからシェル経由でインストールするように案内されるのに、cargo-generateはcargo installを使うのはなぜ？
  - それぞれのGithubのリポジトリのREADME.mdに従っているとするなら納得

資料
---
1. [Wiki](https://ja.wikipedia.org/wiki/WebAssembly)
  - WASMの解説、関連ツールがまとまっている
2. [サーバサイドはWebAssemblyの夢を見るか？ – Node.jsでwasmってみた](https://recruit.gmo.jp/engineer/jisedai/blog/dream-of-wasm/)
3. [Rust by Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/)
4. [WebAssembly ツアー](https://tourofrust.com/webassembly/00_ja.html)
  - スライド形式でWASMの概要を学ぶ
  - コーティングしながら実行できる環境が提供されていて、手を動かしながら学べる
5. [#49 – Mind the Gap: Analyzing the Performance of WebAssembly vs. Native Code](https://misreading.chat/2019/02/18/episode-49-mind-the-gap-analyzing-the-performance-of-webassembly-vs-native-code/)
  - この人はWASMをワスムと発音していた
6. [WebAssemblyはなぜ速いのか](https://postd.cc/what-makes-webassembly-fast/)
7. [プログラミング言語Rust](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/README.html)
8. [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html#the-rust-programming-language-%E6%97%A5%E6%9C%AC%E8%AA%9E%E7%89%88)
[WebAssembly 開発環境構築の本](https://wasm-dev-book.netlify.app/)
9. [Rust and WebASsembly](https://moshg.github.io/rustwasm-book-ja/game-of-life/introduction.html)
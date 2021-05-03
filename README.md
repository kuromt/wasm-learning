

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
  - `{:?}`だとDisplayが実装されていない変数をprintできるのはなぜ？

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

疑問
---

- WASMが扱えるのは4GBまでとあったが、それより大きなモジュールを扱う手段はない？
- WASMはjavascriptより高速とあるが、そのWASMを呼ぶのにjavascriptを書かないといけない設計になっているのはなぜ？
    - メインロジックはWASMで呼び出しだけjavascriptなのはわかる
    - ただ、javascript側もwasmが何の関数をもち、その関数の返り値が何を意味するかを把握した人しか書けない
        - pydocのようなwasmのdocが必要になる？


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

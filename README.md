# このレポジトリについて
[競プロ典型 90問](https://atcoder.jp/contests/typical90)をRustで解きながら、アルゴリズムとRustに関して理解を深めていくレポジトリです．<br>
@sleepyfox97の学習用レポジトリになります．<br>

[競プロ典型 90問]が終わったら、「プログラミングコンテスト攻略のためのアルゴリズムとデータ構造」をやってみる．→そのあと「プログラミングコンテストチャレンジブック」の予定<br>
[arai60](https://1kohei1.com/leetcode/)とかも良いらしい<br>


# 環境周りメモ
```shell
$ rustc --version
rustc 1.79.0 (129f3b996 2024-06-10)
$ cargo --version
cargo 1.79.0 (ffa9cf99a 2024-06-03)
```

# コードの実行方法
各ディレクトリにおいて以下を実行
```shell
$ cargo run
```
標準入力として、問題文に付属の入力例をペーストしてエンターを押すことで実行可能<br>

テストに関しては各ディレクトリで以下のコマンドを実行する<br>
```shell
$ cargo test
```

# 主な個人的にルール
* 型はRustの学習のためにも基本的に明示する．<br>
* 標準入力は基本的に`proconio::input;`を用いて受け取ることにする．<br>
* `proconio::input`以外の外部マクロは使わない．欲しくなったら自作する．<br>
* 標準ライブラリだけで可能な限りコードを書く<br>
* testの実装も行うようにする．<br>

# Rust自体の理解のためのサイト
* [Tour of Rust](https://tourofrust.com/index.html)
* [the book](https://doc.rust-lang.org/book/)
* [rustlings](https://github.com/rust-lang/rustlings/)
* [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

# アルゴリズム周りの学習ツール
* アルゴ式の[データ構造とアルゴリズム](https://algo-method.com/topics)の章をやる．
* LeetCodeの[Blind75](https://leetcode.com/problem-list/oizxjoit/)
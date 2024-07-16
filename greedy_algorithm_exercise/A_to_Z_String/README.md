[A to Z String](https://atcoder.jp/contests/abc053/tasks/abc053_b)<br>

Stringに存在する、[find](https://doc.rust-lang.org/std/string/struct.String.html#method.find)と[rfind](https://doc.rust-lang.org/std/string/struct.String.html#method.rfind)を使うともっと簡単に描ける．<br>
```rust
use proconio::input;

fn main() {
  input!{
    s: String,
  }

  println!("{}", (s.rfind('Z').unwrap() - s.find('A').unwrap() + 1));
}
```
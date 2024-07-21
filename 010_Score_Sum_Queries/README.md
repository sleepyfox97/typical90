## 解答方針
以下のコードでは`O(N^2)`でTLEになってしまうので、累積和を求める方法を用いて、`O(N + Q)`にする．
```rust
use proconio::input;

fn main() {
    input!{
        n: usize,
        cps: [[usize; 2]; n],
        q: usize,
        lrs: [[usize; 2]; q],
    };

    for lr in lrs.iter() {
        let mut sum_o = 0;
        let mut sum_t = 0;
        for i in (lr[0] - 1)..lr[1] {
            if cps[i][0] == 1 {
                sum_o += cps[i][1];
            } else {
                sum_t += cps[i][1];
            }
        }
        println!("{} {}", sum_o, sum_t);
    }
}

```

## 実装メモ
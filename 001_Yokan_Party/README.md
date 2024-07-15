## 解答方針
ある長さで分割ができるかを二分探索で絞り込む．<br>

> [!TIP]
> 「最小値を最大にする」みたいな時には二分探索を用いるとうまくいくことがある．


## 実装メモ
### 標準入力の受け取り
* `proconio::input`を用いると良い．

### 二分探索
二分探索のデモコードとしては以下のようなコードが考えられる．これを発展させていけばOK<br>
```rust
    let mut min: usize = 1;
    let mut max: usize = l;
    while max - min > 1 {
        if check_function(max + min) / 2 == true {
            lower = (max + min) / 2;
        } else {
            upper = (max + min) / 2;
        }
    }
```
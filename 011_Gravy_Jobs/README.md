## 解答方針
2^Nのパターンに関して全探索で、N<20以下くらいまでなら、終わりそう．<br>
制約なしの場合に対応するためには、DPで考えると良い．<br>
DPテーブルは、N * 最大の締切日で考える<br>
この場合、計算量はN^2となるが、制約的に5000 * 5000の計算量が最大となり、ギリギリ2Sで終わる可能性が見えてくる．

以下のコードだと、N<=20に関しては特定可能．<br>
```rust
use proconio::input;

fn main() {
    input!{
        n: usize,
        mut dcs: [[usize; 3]; n],
    };

    // 締め切りが早い順に並べる
    dcs.sort_by(|a, b| {
        a[0].cmp(&b[0])
    });

    //各パターンに関して仕事が完了できるかを見る．
    let mut re: usize = 0;
    for bit in 0..(1<<n) {
        let mut day: usize = 0;
        let mut tmp: usize = 0;
        println!("{}", bit);
        for i in 0..n {
            println!("{}", bit & (1 << i));
            if (bit & (1 << i)) != 0{
                day += dcs[i][1];
                if day > dcs[i][0] {
                    tmp = 0;
                    break;
                }
                tmp += dcs[i][2];
            }
        }
        re = re.max(tmp);
    }
    println!("{}", re);
}
```

## 実装メモ

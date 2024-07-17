## 解答方針
辞書順で最小のものを出力するので貪欲法で実装していく．<br>

## 実装メモ
### 貪欲法
辞書順に前の方の文字から順番に調べていく．

### 文字列のi文字目以降で文字Cが出現する最小のindexを保存した配列の作成
このタイプの配列を前処理として用意しておくのは結構典型っぽいのでなれておくと良いらしい．<br>
```rust
fn calc_next(s: &str, n: usize) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = vec![vec![n; 26]; n + 1];

    for i in (0..n).rev() {
        // i + 1 文字目以降の結果を一旦 i 文字にコピー
        res[i] = res[i + 1].clone();

        // i 文字目の情報を反映させる
        let index = s.as_bytes()[i] as usize - 'a' as usize;
        res[i][index] = i;
    }

    res
}
```

### RMQ
インデックス付きRMQ(Range Minimum Query)を使うとNlog(N)くらいで終わるらしいので、そこに関して調べる
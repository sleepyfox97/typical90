//このコード
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
        for i in 0..n {
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

//以下のDPの解法で時間内に解き終わる
// use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         mut dcs: [[usize; 3]; n],
//     };

//     // 締め切りが早い順に並べる
//     dcs.sort_by(|a, b| a[0].cmp(&b[0]));

//     // 最大の締め切り日を取得
//     let max_deadline = dcs.iter().map(|x| x[0]).max().unwrap();

//     // DPテーブルを作成し、初期化
//     let mut dp = vec![vec![0; max_deadline + 1]; n + 1];

//     for i in 0..n {
//         let (d, c, s) = (dcs[i][0], dcs[i][1], dcs[i][2]);
//         for j in 0..=max_deadline {
//             // 仕事iを選ばない場合
//             dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
//             // 仕事iを選ぶ場合
//             if j + c <= d {
//                 dp[i + 1][j + c] = dp[i + 1][j + c].max(dp[i][j] + s);
//             }
//         }
//     }

//     // 最後の行の最大値が求める報酬の最大値
//     let max_reward = dp[n].iter().max().unwrap();
//     println!("{}", max_reward);
// }

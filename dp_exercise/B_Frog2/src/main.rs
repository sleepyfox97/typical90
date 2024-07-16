use proconio::input;

fn chmin(a: i64, b: i64) -> i64 {
    if a > b {
        return b;
    }
    return a;
}

fn main() {
    input!{
        n: usize,
        k: usize,
        h_n: [i64; n],
    }

    let mut cost: Vec<i64> = vec![std::i64::MAX; n];
    cost[0] = 0;
    for i in 0..n {
        for j in 1..(k + 1) {
            if j + i >= n {
                continue;
            }
            cost[j + i] = chmin(cost[j + i], (h_n[j + i] - h_n[i]).abs() + cost[i]);
        }
    }
    println!("{}", cost[n - 1]);
}
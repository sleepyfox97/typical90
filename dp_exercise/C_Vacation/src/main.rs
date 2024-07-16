use proconio::input;

fn chmax(a: i64, b: i64) -> i64 {
    if a < b {
        return b;
    }
    return a;
}

fn main() {
    input!{
        n: usize,
        score: [[i64; 3]; n],
    }

    let mut cost: Vec<Vec<i64>> = vec![vec![0; 3]; n];

    cost[0][0] = score[0][0];
    cost[0][1] = score[0][1];
    cost[0][2] = score[0][2];
    for i in 1..n {
        cost[i][0] = chmax(cost[i][0], cost[i - 1][1] + score[i][0]);
        cost[i][0] = chmax(cost[i][0], cost[i - 1][2] + score[i][0]);
        cost[i][1] = chmax(cost[i][1], cost[i - 1][0] + score[i][1]);
        cost[i][1] = chmax(cost[i][1], cost[i - 1][2] + score[i][1]);
        cost[i][2] = chmax(cost[i][2], cost[i - 1][0] + score[i][2]);
        cost[i][2] = chmax(cost[i][2], cost[i - 1][1] + score[i][2]);
    }
    println!("{}", cost[n-1].iter().max().unwrap());
}
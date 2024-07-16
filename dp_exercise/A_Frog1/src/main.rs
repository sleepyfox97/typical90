use proconio::input;

fn main() {
    input!{
        n: usize,
        h_n: [i64; n],
    }

    let mut cost: Vec<i64> = vec![0; n];
    cost[1] = (h_n[1] - h_n[0]).abs();
    for i in 2..n {
        if (cost[i - 1] + (h_n[i] - h_n[i - 1]).abs()) < (cost[i - 2] + (h_n[i] - h_n[i - 2]).abs()) {
            cost[i] = cost[i - 1] + (h_n[i] - h_n[i - 1]).abs();
        } else {
            cost[i] = cost[i - 2] + (h_n[i] - h_n[i - 2]).abs();
        }
    }
    println!("{}", cost[n - 1]);
}
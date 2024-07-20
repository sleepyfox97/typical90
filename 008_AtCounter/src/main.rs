use proconio::input;

fn add(a: usize, b: usize) -> usize {
    (a + b) % (1000000007)
}

fn main() {
    input!{
        n: usize,
        s: String,
    };

    let atcoder = "atcoder";
    let mut dp: Vec<Vec<usize>> = vec![vec![0; atcoder.len() + 1]; n + 1];

    let s_bytes = s.as_bytes();
    let atcoder_bytes = atcoder.as_bytes();
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=(atcoder.len()) {
            dp[i+1][j] = add(dp[i+1][j], dp[i][j]);
            if (j < atcoder.len()) && (s_bytes[i] == atcoder_bytes[j]) {
                dp[i+1][j+1] = add(dp[i+1][j+1], dp[i][j]);
            }
        }
    }
    println!("{}",dp[n][atcoder.len()]);
}
use proconio::input;

fn chmax(a: usize, b: usize) -> usize {
    if a < b {
        return b;
    }
    return a;
}

fn main() {
    input!{
        n: usize,
        w: usize,
        goods: [[usize; 2]; n],
    }

    let mut score: Vec<Vec<usize>> = vec![vec![0; w+1]; n + 1];
    for i in 0..n {
        for sum_w in (0..(w+1)).rev() {
            if sum_w >= goods[i][0]{
                score[i + 1][sum_w] = chmax(score[i + 1][sum_w], score[i][sum_w - goods[i][0]] + goods[i][1]);
            }
            score[i + 1][sum_w] = chmax(score[i + 1][sum_w], score[i][sum_w]);
        }
    }

    println!("{}", score[n][w]);
}
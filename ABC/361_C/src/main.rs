use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut ai: [usize; n],
    };

    ai.sort();
    let mut ans: usize = std::usize::MAX;
    // ここの=kを忘れててハマった．
    for i in 0..=k {
        ans = if ai[n - 1 - (k - i)] - ai[i] < ans {
            ai[n - 1 - (k - i)] - ai[i]
        } else {
            ans
        };
    }

    println!("{}", ans);
}
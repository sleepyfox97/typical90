use proconio::input;

fn check (ai: &Vec<usize>, mid: usize, m: usize) -> bool {
    let mut sum = 0;
    for a in ai.iter() {
        sum += a.min(&mid);
    }
    sum <= m
}

fn main() {
    input!{
        n: usize,
        m: usize,
        mut ai: [usize; n],
    };

    ai.sort();

    let mut l = 0;
    let mut r = ai[n - 1] + 1;
    while l < r {
        let mid = (l + r + 1) / 2;
        if check(&ai, mid, m) {
            l = mid;
        } else {
            r = mid - 1;
        }
    }
    if l >= ai[n - 1] {
        println!("infinite");
    } else {
        println!("{}", l);
    }
}

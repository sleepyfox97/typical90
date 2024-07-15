use proconio::input;

fn is_valid(i: usize, n: usize) -> (bool, String) {
    let mut score: i64 = 0;
    let mut s: String = String::from("");
    for j in (0..n).rev() {
        if (i >> j) & 1 == 0{
            score += 1;
            s += "(";
        } else {
            score -= 1;
            s += ")";
        }
        if score < 0 {
           return (false, s);
        }
    }
    (score == 0, s)
}

fn main() {
    input!{
        n: usize,
    }

    let mut v: bool;
    let mut s: String;

    for i in 0..(1 << n) {
        (v, s) = is_valid(i, n);
        if v == true {
            println!("{}", s);
        }
    }
}
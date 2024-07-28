use proconio::input;
use permutohedron::LexicalPermutation;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut s: proconio::marker::Chars,
    };

    let mut ans = 0;
    // let ps = s.iter().permutations(n).unique(); //これを使うと、全パターンがpsに書き込まれ、メモリの消費量も多くなる．その分時間もかかる
    s.sort();
    loop {
        let is_first = s.next_permutation();
        let mut is_ok = true;
        for i in 0..n-k+1 {
            let mut is_palindrome = true;
            for j in 0..k/2 {
                if s[i + j] != s[i + k - j - 1] {
                    is_palindrome = false;
                    break;
                }
            }
            if is_palindrome {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            ans += 1;
        }
        if !is_first {
            break;
        }
    }

    println!("{}", ans);
}
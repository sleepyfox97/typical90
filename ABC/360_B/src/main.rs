use proconio::input;

fn main() {
    input!{
        s: String,
        t: String,
    };

    let s_len = s.len();
    let t_len = t.len();

    for w in 1..s_len {
        for c in 0..w {
            let mut now = String::new();
            let mut i = c;

            while i < s_len {
                now.push(s.chars().nth(i).unwrap());
                i += w;
            }

            if now == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");

}
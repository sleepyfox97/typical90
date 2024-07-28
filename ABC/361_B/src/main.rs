use proconio::input;

fn main() {
    input!{
        a: (usize, usize, usize, usize, usize, usize),
        b: (usize, usize, usize, usize, usize, usize),
    };

    if (b.0 < a.3 && b.1 < a.4 && b.2 < a.5) && (a.0 < b.3 && a.1 < b.4 && a.2 < b.5) {
        println!("Yes");
    } else {
        println!("No");
    }
}
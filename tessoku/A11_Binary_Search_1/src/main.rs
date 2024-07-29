use proconio::input;

fn main() {
    input!{
        n: usize,
        x: usize,
        a: [usize; n],
    };

    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        let mid = (l + r) / 2;
        if a[mid] < x {
            l = mid + 1;
        } else {
            r = mid ;
        }
    }

    println!("{}", l + 1);
}
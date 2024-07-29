use proconio::input;

fn main() {
    input!{
        n: usize,
        x: usize,
        a: [usize; n],
    };

    let mut l = 1;
    let mut r = n;
    while l <= r {
        let mid = (l + r) / 2;
        if a[mid] < x {
            l = mid + 1;
        }
        if a[mid] == x{
            l = mid;
            break;
        }
        if a[mid] > x{
            r = mid - 1;
        }
    }

    println!("{}", l);
}
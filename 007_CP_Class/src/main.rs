use proconio::input;

fn main() {
    input!{
        n: usize,
        mut a_n: [usize; n],
        q: usize,
        b_q: [usize; q],
    }

    a_n.sort();

    for v in b_q.iter() {
        let mut upper: usize = n - 1;
        let mut lower: usize = 0;

        while upper - lower > 1 {
            let mid: usize = (upper + lower) / 2;
            if v < &a_n[mid] {
                upper = mid;
            } else {
                lower = mid;
            }
        }
        let diff_a = (a_n[lower] as isize - *v as isize).abs() as usize;
        let diff_b = (a_n[upper] as isize - *v as isize).abs() as usize;
        if diff_a < diff_b {
            println!("{}", diff_a);
        } else {
            println!("{}", diff_b);
        }
    }
}
use proconio::input;

fn can_cut(l: usize, k: usize, a_i: &Vec<usize>, length: usize) -> bool {
    let mut count: usize = 0;
    let mut current_position: usize = 0;

    for &a in a_i.iter() {
        if a - current_position >= length {
            current_position = a;
            count += 1;
        }
    }

    if l - current_position >= length {
        count += 1;
    }
    count >= (k + 1)
}

fn main() {
    input!{
        n: usize,
        l: usize,
        k: usize,
        a_i: [usize; n],
    }

    let mut upper: usize = l;
    let mut lower: usize = 1;
    while upper - lower > 1 {
        if can_cut(l, k, &a_i, (lower + upper) / 2) == true {
            lower = (lower + upper) / 2;
        } else {
            upper = (lower + upper) / 2;
        }
    }
    println!("{}",  lower)
}
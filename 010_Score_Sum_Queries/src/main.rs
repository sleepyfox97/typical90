use proconio::input;

fn main() {
    input!{
        n: usize,
        cps: [[usize; 2]; n],
        q: usize,
        lrs: [[usize; 2]; q],
    };

    let mut sum_o_list: Vec<usize> = vec![0; 1];
    let mut sum_t_list: Vec<usize> = vec![0; 1];
    for cp in cps.iter() {
        if cp[0] == 1 {
            sum_o_list.push(*sum_o_list.last().unwrap() + cp[1]);
            sum_t_list.push(*sum_t_list.last().unwrap());
        } else {
            sum_o_list.push(*sum_o_list.last().unwrap());
            sum_t_list.push(*sum_t_list.last().unwrap() + cp[1]);
        }
    }

    for lr in lrs.iter() {
        let sum_o = sum_o_list[lr[1]] - sum_o_list[lr[0] - 1];
        let sum_t = sum_t_list[lr[1]] - sum_t_list[lr[0] - 1];
        println!("{} {}", sum_o, sum_t);
    }
}

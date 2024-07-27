use proconio::input;

fn main(){
    input!{
        n: usize,
        rs: [i64; n],
    };

    let mut min = rs[0];
    let mut max = -200000000;
    for i in 1..n {
        if max < rs[i] - min {
            max = rs[i] - min;
        }
        if min > rs[i] {
            min = rs[i];
        }
    }
    println!("{}", max);
}
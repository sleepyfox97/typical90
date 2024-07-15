use proconio::input;

fn main(){
    input!{
        h: usize,
        w: usize,
        m: [[usize; w]; h],
    }

    let mut h_sum: Vec<usize> = vec![0; h];
    let mut w_sum: Vec<usize> = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            h_sum[i] += m[i][j];
            w_sum[j] += m[i][j];
        }
    }

    for  i in 0..h {
        for j in 0..w {
            print!("{} ", h_sum[i] + w_sum[j] - m[i][j]);
        }
        println!();
    }
}
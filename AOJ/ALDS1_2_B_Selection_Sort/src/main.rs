use proconio::input;

fn main(){
    input!{
        n: usize,
        mut rs: [usize; n],
    };

    let mut sw = 0;
    let mut min = 0;
    for i in 0..n {
        min = i;
        //indexがi ~ nの範囲で最小値のindexを求める
        for j in i..n {
            if rs[min] > rs[j] {
                min = j;
            }
        }
        rs.swap(i, min);
        if i != min {
            sw += 1;
        }
        println!("i = {}, {:?}", i, rs);
    }
    println!("{:?}", rs);
    println!("{}", sw);
}
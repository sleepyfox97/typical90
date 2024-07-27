use proconio::input;

fn main(){
    input!{
        n: usize,
        mut rs: [usize; n],
    };

    let mut sw = 0;
    for i in 0..n {
        let mut flag = 0;
        //バブルソートは後ろから見ていく
        for j in ((i+1)..n).rev() {
            if rs[j] < rs[j - 1] {
                rs.swap(j, j - 1);
                flag = 1;
                sw += 1;
                println!("i = {}, j = {} {:?}",i, j, rs);
            }
        }
        if flag == 0 {
            break;
        }
    }
    println!("{:?}", rs);
    println!("{}", sw);
}
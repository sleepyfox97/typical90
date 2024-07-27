use proconio::input;

fn main(){
    input!{
        n: usize,
        mut rs: [usize; n],
    };

    for i in 1..n {
        //今見てるカード
        let v = rs[i];
        //自分のカードより前にあるカードの中で適切な場所におく
        let mut j = i;
        while j  > 0 && rs[j - 1] > v{
            rs[j] = rs[j - 1];
            j -= 1;
        }
        rs[j] = v;
        println!("{:?}", rs);
    }
}
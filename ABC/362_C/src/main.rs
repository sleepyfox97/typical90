use proconio::input;

fn main() {
    input!{
        n: usize,
        lrs: [[i64; 2];n],
    };

    let mut l_sum: i64 = 0;
    let mut r_sum: i64 = 0;
    let mut ans: Vec<i64> = vec![];
    for lr in lrs.iter() {
        l_sum += lr[0];
        r_sum += lr[1];
        ans.push(lr[0]);
    }
    if l_sum > 0 || r_sum < 0 {
        println!("No");
        return ;
    }

    for i in 0..n {
        let d = if lrs[i][1] - lrs[i][0] < (-1) * l_sum {
            lrs[i][1] - lrs[i][0]
        } else {
            (-1) * l_sum
        };
        l_sum += d;
        ans[i] += d;
    }
    println!("Yes");
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
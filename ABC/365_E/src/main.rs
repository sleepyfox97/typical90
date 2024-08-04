use proconio::input;

fn main() {
    input!{
        n: usize,
        ai: [usize; n],
    };

    let mut ans = 0;
    for i in 0..30 {
        // 0の個数は最初から1つにしておかないと、1ののみの時の区間がおかしくなる
        let mut count_0 = 1;
        let mut count_1 = 0;
        let mut tmp = 0;
        for a in ai.iter() {
            tmp = tmp ^ ((a >> i) & 1);
            if tmp == 1 {
                count_1 += 1;
            } else {
                count_0 += 1;
            }
        }
        ans += count_0 * count_1 * (1 << i);
    }
    
    println!("{}", ans - ai.iter().sum::<usize>());
}

use proconio::input;

fn calc_next(s: &str, n: usize) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = vec![vec![n; 26]; n + 1];

    for i in (0..n).rev() {
        // i + 1 文字目以降の結果を一旦 i 文字にコピー
        res[i] = res[i + 1].clone();

        // i 文字目の情報を反映させる
        let index = s.as_bytes()[i] as usize - 'a' as usize;
        res[i][index] = i;
    }

    res
}

fn main() {
    input!{
        n: usize,
        k: usize,
        s: String,
    }

    let nex: Vec<Vec<usize>> =  calc_next(s.as_str(), n);

    //貪欲法．二重ループの内側で、aから順番に調べる．
    let mut res = String::new();
    let mut j: isize = -1;
    for i in 0..k {
        for c in 'a'..='z' {
            let m: usize = nex[(j + 1) as usize][(c as u8 - 'a' as u8) as usize];

            // ちゃんと K 文字が作れれば OK
            if n - m >= k - i {
                res.push(c);
                j = m as isize;
                break;
            }
        }
    }
    println!("{}", res);
}
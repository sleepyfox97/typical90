use proconio::input;

fn main() {
  input!{
    n: usize,
    k: usize,
    ai: [usize; n],
  };
  
  let mut ans = 0;
  let mut l = 0;
  for r in 1..n {
    if ai[r] - ai[l] <= k {
      ans += r - l;
    } else {
      while ai[r] - ai[l] > k {
        l += 1;
      }
      ans += r - l;
    }
  }
  println!("{}", ans);
}
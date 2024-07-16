use proconio::input;

fn main() {
  input!{
    s: String,
  }

  let mut place_a: usize = 0;
  let mut a_flag: bool = false;
  let mut place_z: usize = 0;
  for (i, v) in s.chars().enumerate() {
    if v == 'A' && a_flag == false {
        place_a = i;
        a_flag = true;
    }
    if v == 'Z'{
        place_z = i;
    }
  }
  println!("{}", (place_z - place_a + 1))
}
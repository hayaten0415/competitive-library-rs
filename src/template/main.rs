// 競プロ用なのでResultでOkのケースが得られるとする
#[allow(dead_code)]
pub fn input<T: std::str::FromStr>() -> T {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).ok();
  input.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
pub fn input_t2<T: std::str::FromStr, U: std::str::FromStr>() -> (T, U) {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).ok();
  let s = input.trim().split_whitespace().collect::<Vec<&str>>();
  (s[0].parse().ok().unwrap(), s[1].parse().ok().unwrap())
}

#[allow(dead_code)]
pub fn input_vec<T: std::str::FromStr>() -> Vec<T> {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).ok();
  input.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect()
}

#[allow(dead_code)]
fn chmin<T: std::cmp::PartialOrd>(a1 : &mut T, a2 : T) -> bool {
  if *a1 > a2 {*a1 = a2; true} else {false}
}

#[allow(dead_code)]
fn chmax<T: std::cmp::PartialOrd>(a1 : &mut T, a2 : T) -> bool {
  if *a1 < a2 {*a1 = a2; true} else {false}
}
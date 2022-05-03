pub fn euler_phi(mut n: u64) -> u64 {
  let mut res = n;
  let mut i = 2;
  while i * i <= n {
    if n % i == 0 {
      res -= res / i;
      while n % i == 0 {n /= i;}
    }
    i += 1;
  }
  if n > 1 {res -= res / n;}
  res
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_euler_phi() {
    assert_eq!(euler_phi(15), 8);
    assert_eq!(euler_phi(105), 48);
    assert_eq!(euler_phi(180), 48);
    assert_eq!(euler_phi(625), 500);
  }
}
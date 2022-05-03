pub fn divisor(n: u64)-> Vec<u64> {
  let mut divisor = vec![];
  let mut i = 1;
  while i * i <= n {
    if n % i == 0 {
      divisor.push(i);
      if i * i != n {divisor.push(n / i);}
    }
    i += 1;
  }
  divisor.sort();
  divisor
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_divisor() {
    assert_eq!(divisor(12), [1, 2, 3, 4, 6, 12]);
    assert_eq!(divisor(1), [1]);
    assert_eq!(divisor(35), [1, 5, 7, 35]);
  }
}
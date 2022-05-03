pub fn euler_phi_table(n: usize)-> Vec<u64> {
  let mut euler = vec![0; n + 1];
  for i in 0..n+1 {
    euler[i] = i as u64;
  }
  let mut i = 2;
  while i <= n {
    if euler[i] == i as u64 {
      let mut j = i;
      while j <= n {
        euler[j] = euler[j] / (i as u64) * (i - 1) as u64;
        j += i;
      }
    }
    i+= 1;
  }
  euler
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_euler_phi() {
    assert_eq!(euler_phi_table(11), [0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10]);
  }
}
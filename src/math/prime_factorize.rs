pub fn prime_factorize(mut n: u64)-> std::collections::HashMap<u64, u64> {
  let mut primes = std::collections::HashMap::new();
  let mut i = 2;
  while i * i <= n {
    while n % i == 0 {
      *primes.entry(i).or_insert(0) += 1;
      n /= i;
    }
    i += 1;
  }
  if n > 1 {
    *primes.entry(n).or_insert(0) += 1;
  }
  primes
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_prime_factorize() {
    assert_eq!(prime_factorize(24).get(&2), Some(&3));
    assert_eq!(prime_factorize(24).get(&3), Some(&1));
    assert_eq!(prime_factorize(7).get(&7), Some(&1));
    assert_eq!(prime_factorize(7).get(&5), None);
  }
}
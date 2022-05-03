#[derive(Debug)]
pub struct SieveEratosthenes {
  f: Vec<i64>,
  primes: Vec<i64>,
  n: usize,
}

impl SieveEratosthenes {
  pub fn new(n: usize) -> Self {
    let mut f : Vec<i64> =  vec![0; n+1];
    let mut primes: Vec<i64> = vec![];
    f[0] = -1;
    f[1] = -1;
    for i in 2..n + 1 {
      if f[i] == 0 {
        primes.push(i as i64);
        let mut j = i;
        while j <= n {
          if f[j] == 0 {
            f[j] = i as i64;
          }
          j += i;
        }
      }
    }
    Self { f, primes, n}
  }

  pub fn is_prime(&self, x : usize) -> bool {
    self.f[x] == x as i64
  }

  pub fn factorlist(&self, mut x : usize) -> Vec<i64> {
    assert!(x as usize > 0);
    let mut res = vec![];
    while x > 1 {
      res.push(self.f[x]);
      x /= self.f[x] as usize;
    }
    res
  }

  pub fn factor(&self, x: usize) -> Vec<(i64, i64)> {
    let fl = self.factorlist(x);
    if fl.len() == 0 {return vec![];}
    let mut res = vec![(fl[0], 0); 1];
    for p in fl.iter() {
      let d = res.len();
      if res[d-1].0 == *p {
        res[d-1].1 += 1;
      } else {
        res.push((*p, 1));
      }
    }
    res
  }

  pub fn divisor(&self, x: usize)-> Vec<i64> {
    assert!(x > 0);
    let mut res = vec![1; 1];
    for (prime, ex) in self.factor(x) {
      let n = res.len() * ex as usize;
      for i in 0..n {
        res.push(res[i] * prime);
      }
    }
    res.sort();
    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_primelist() {
    let sieve = SieveEratosthenes::new(100);
    assert_eq!(sieve.primes, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
  }

  #[test]
  fn test_divisor() {
    let sieve = SieveEratosthenes::new(100);
    assert_eq!(sieve.divisor(12), [1, 2, 3, 4, 6, 12])
  }

  #[test]
  fn test_factor() {
    let sieve = SieveEratosthenes::new(100);
    assert_eq!(sieve.factor(12), [(2, 2), (3, 1)]);
    assert_eq!(sieve.factor(42), [(2, 1), (3, 1), (7, 1)]);
  }
}
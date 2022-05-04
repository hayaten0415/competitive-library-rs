pub struct Dsu {
  n: usize,
  par: Vec<i64>,
}

impl Dsu {
  pub fn new(n: usize) -> Self {
    Self {
      n,
      par: vec![-1; n],
    }
  }

  pub fn merge(&mut self, x: usize, y: usize) -> usize {
    assert!(x < self.n);
    assert!(y < self.n);
    let (mut a, mut b) = (self.leader(x), self.leader(y));
    if a == b { return a;}
    if -self.par[a] < -self.par[b] {
      std::mem::swap(&mut a, &mut b);
    }
    self.par[a] += self.par[b];
    self.par[b] = a as i64;
    a
  }

  pub fn leader(&mut self, x: usize) -> usize {
    assert!(x < self.n);
    if self.par[x] < 0 {
      return x
    }
    self.par[x] = self.leader(self.par[x] as usize) as i64;
    self.par[x] as usize
  }

  pub fn same(&mut self, x: usize, y: usize) -> bool {
    assert!(x < self.n);
    assert!(y < self.n);
    self.leader(x) == self.leader(y)
  }

  pub fn size(&mut self, x: usize) -> usize {
    assert!(x < self.n);
    let a = self.leader(x);
    -self.par[a] as usize
  }

  pub fn groups(&mut self) -> Vec<Vec<usize>> {
    let mut leader_buf = vec![0; self.n];
    let mut group_size = vec![0; self.n];
    for i in 0..self.n {
      leader_buf[i] = self.leader(i);
      group_size[leader_buf[i]] += 1;
    }
    let mut result = vec![vec![]; self.n];
    for i in 0..self.n {
      result[i].reserve(group_size[i]);
    }
    for i in 0..self.n {
      result[leader_buf[i]].push(i);
    }
    result
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<Vec<usize>>>()
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn dsu_test() {
    let mut d = Dsu::new(5);
    d.merge(0, 1);
    assert!(d.same(0, 1));
    d.merge(1, 2);
    assert!(d.same(0, 2));
    assert_eq!(d.size(0), 3);
    assert!(!d.same(0, 3));
    d.merge(3, 4);
    assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3, 4]]);
  }
}
pub fn toposort(g: &Vec<Vec<usize>>) -> Vec<usize> {
  let n = g.len();
  let mut ans = vec![];
  let mut ind = vec![0; n];
  for i in 0..n {
    for &v in g[i].iter() {
      ind[v] += 1;
    }
  }
  let mut que: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
  for i in 0..n {
    if ind[i] == 0 {que.push_back(i);}
  }
  while let Some(i) = que.pop_front() {
    ans.push(i);
    for &e in g[i].iter() {
      ind[e] -= 1;
      if ind[e] == 0 {que.push_back(e);}
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn toposort_test() {
    let mut g: Vec<Vec<usize>> = vec![vec![]; 4];
    g[0].push(1);
    g[2].push(0);
    g[2].push(1);
    g[2].push(3);
    g[3].push(0);
    assert_eq!(toposort(&g), vec![2, 3, 0, 1]);
  }
}
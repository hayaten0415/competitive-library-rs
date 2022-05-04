pub fn lcs(s: String, t: String) -> i32 {
  let n = s.len();
  let m = t.len();
  let mut dp = vec![vec![0; m + 1]; n+1];
  let s1: Vec<char> = s.chars().collect();
  let t1: Vec<char> = t.chars().collect();
  for i in 0..n {
    for j in 0..m {
      if s1[i] == t1[j] {
        dp[i+1][j+1] = dp[i][j] + 1;
      } else{
        dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
      }
    }
  }
  dp[n][m]
}

// verify https://atcoder.jp/contests/dp/tasks/dp_f
pub fn lcs_restore(s: String, t: String) -> String {
  let n = s.len();
  let m = t.len();
  let mut dp = vec![vec![0; m + 1]; n+1];
  let s1: Vec<char> = s.chars().collect();
  let t1: Vec<char> = t.chars().collect();
  for i in 0..n {
    for j in 0..m {
      if s1[i] == t1[j] {
        dp[i+1][j+1] = dp[i][j] + 1;
      } else{
        dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
      }
    }
  }
  let mut ans = String::new();
  let mut i = n;
  let mut j = m;
  while i > 0 && j > 0 {
    if dp[i][j] == dp[i-1][j] {i -= 1;}
    else if dp[i][j] == dp[i][j-1] { j-= 1; }
    else {
      ans.push(s1[i-1]);
      i -= 1;
      j -= 1;
    }
  }
  ans = ans.chars().rev().collect::<String>();
  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lcs() {
    assert_eq!(lcs(String::from("axyb"), String::from("abyxb")), 3);
    assert_eq!(lcs(String::from("aa"), String::from("xayaz")), 2);
    assert_eq!(lcs_restore(String::from("aa"), String::from("xayaz")), "aa");
  }
}
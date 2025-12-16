fn main() {
    println!("Hello, world!");
    println!("{}", check_partitioning("abcbdd".to_string()))
}

fn check_partitioning(s: String) -> bool {
    let n = s.len();
    let mut dp = vec![vec![false; n]; n];
    for l in 1..n {
        for start in 0..=n - l {
            let end = start + l - 1;
            if l == 1 {
                dp[start][end] = true;
            } else if l == 2 {
                dp[start][end] = s.chars().nth(start) == s.chars().nth(end);
            } else {
                dp[start][end] = s.chars().nth(start) == s.chars().nth(end) && dp[start + 1][end - 1];
            }
        }
    }
    for start in 1..n - 1 {
        if !dp[0][start - 1] {
            continue;
        }
        for end in start..n - 1 {
            if dp[start][end] && dp[end + 1][n - 1] {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    println!("Hello, world!");
}


fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut dp = vec![vec![0; k as usize]; k as usize];
    for num in nums {
        let m = num % k;
        for prev in 0..k {
            dp[prev as usize][m as usize] = dp[m as usize][prev as usize] + 1;
            res = res.max(dp[prev as usize][m as usize]);
        }
    }
    res
}
fn main() {
    println!("Hello, world!");
}

fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
    let mut cnt = 1;
    let mut pre_cnt = 0;
    let mut ans = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i-1] {
            cnt += 1;
        } else {
            pre_cnt = cnt;
            cnt = 1;
        }
        ans = ans.max(cnt.min(pre_cnt));
        ans = ans.max(cnt / 2);
    }
    return ans;
}
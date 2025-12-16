fn main() {
    let nums = vec![431,922,158,60,192,14,788,146,788,775,772,792,68,143,376,375,877,516,595,82,56,704,160,403,713,504,67,332,26];
    let max_operations = 80;
    let lowest_cost = minimum_size(nums, max_operations);
    print!("lowest_cost: {}", lowest_cost);
}

fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    let mut left = 0;
    let mut right = *nums.iter().max().unwrap();
    while left + 1 < right {
        let mid = (left + right) / 2;
        if check(mid, &nums, max_operations) {
            right = mid;
        } else {
            left = mid;
        }
    }
    return right;
}

fn check(m: i32, nums: &Vec<i32>, max_operations: i32) -> bool {
    let mut cnt = 0;
    for &num in nums.iter() {
        if num <= m {
            continue;
        } else {
            cnt += (num + 1) / m;
        }
    }
    return cnt <= max_operations;
}
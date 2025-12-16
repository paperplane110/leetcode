use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut mx = 0;
    let mut left = 0;
    let mut tmp = HashSet::<i32>::new();
    for right in 0..nums.len() {
        while let Some(&_) = tmp.get(&nums[right]) {
            tmp.remove(&nums[left]);
            sum -= nums[left];
            left += 1;
        }
        tmp.insert(nums[right]);
        sum += nums[right];
        mx = mx.max(sum);
    }
    mx
}
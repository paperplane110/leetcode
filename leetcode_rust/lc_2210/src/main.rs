fn main() {
    println!("Hello, world!");
}

fn count_hill_valley(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut left = nums[0];
    let mut ans = 0;
    for i in 1..(n - 1) {
        let curr = nums[i];
        let right = nums[i + 1];
        if curr == right {
            continue;
        } else if (right > curr && left > curr) || (right < curr && left < curr) {
            ans += 1;
        }
        left = curr;
    }
    ans
}

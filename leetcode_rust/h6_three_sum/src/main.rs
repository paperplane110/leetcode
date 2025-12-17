use core::num;

fn main() {
    println!("Hello, world!");
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let mut ans: Vec<Vec<i32>> = Vec::new();
    for i in 0..nums.len() - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left: usize = i + 1;
        let mut right: usize = nums.len() - 1;
        let target = -nums[i];

        while left < right {
            let two_sum = nums[left] + nums[right];
            if two_sum == target {
                ans.push(vec![nums[i], nums[left], nums[right]]);

                let val_l = nums[left];
                while left < right && nums[left] == val_l {
                    left += 1;
                }

                let val_r = nums[right];
                while left < right && nums[right] == val_r {
                    right -= 1;
                }
            } else if two_sum > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
    }
    ans
}

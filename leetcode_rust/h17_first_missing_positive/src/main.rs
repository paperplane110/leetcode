fn main() {
    println!("Hello, world!");
}

fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    for i in 0..nums.len() {
        while 1 <= nums[i] && nums[i] <= nums.len() as i32 && nums[i] != nums[nums[i] as usize - 1] {
            let j = nums[i] as usize - 1;
            nums.swap(i, j);
        }
    }

    for i in 0..nums.len() {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }

    return nums.len() as i32 + 1;
}
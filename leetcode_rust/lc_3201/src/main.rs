fn main() {
    println!("Hello, world!");
}

fn max_length(nums: Vec<i32>) -> i32 {
    let mut odd_l = 0;
    let mut even_l = 0;
    let mut alter_l = 1;
    let mut pre_num = nums[0];
    for num in nums.iter() {
        if *num % 2 == 0 {
            even_l += 1;
        } else {
            odd_l += 1;
        }
        if (num - pre_num) % 2 == 1 {
            pre_num = *num;
            alter_l += 1;
        }
    }
    return (odd_l.max(even_l)).max(alter_l);
}

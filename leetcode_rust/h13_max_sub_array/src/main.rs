
fn main() {
    println!("Hello, world!");
}

fn sub_max_array(nums: Vec<i32>) -> i32 {
    let mut pre = 0;
    let mut ans = nums[0];
    for &n in nums.iter() {
        pre = n.max(pre + n);
        ans = ans.max(pre);
    }
    ans
}
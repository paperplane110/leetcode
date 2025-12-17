use std::mem::swap;

fn main() {
    println!("Hello, world!");
}

fn move_zeros(nums: &mut Vec<i32>) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
        i += 1;
    }
}
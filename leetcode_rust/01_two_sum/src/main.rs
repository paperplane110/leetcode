use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let res = two_sum(nums, target);
    println!("{:?}", res);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_table = HashMap::<i32, i32>::new();
    for (i, num) in nums.iter().enumerate() {
        if let Some(&index) = hash_table.get(&(target - num)) {
            return vec![index, i as i32];
        }
        hash_table.insert(*num, i as i32);
    }
    return vec![];
}

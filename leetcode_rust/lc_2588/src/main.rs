use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn beautiful_sub_arrays(nums: Vec<i32>) -> i64 {
    let mut cnt: HashMap<i32, i64> = HashMap::from([(0, 1)]);
    let mut res: i64 = 0;
    let mut mask: i32 = 0;
    for num in nums {
        mask ^= num;
        res += cnt.get(&mask).copied().unwrap_or(0);
        *cnt.entry(mask).or_insert(0) += 1
    }
    return res;
}
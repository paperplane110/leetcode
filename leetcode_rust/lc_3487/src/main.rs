use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}


fn max_sum(nums: Vec<i32>) -> i32 {
    let mut mx = 0;
    let mut flag = false;
    let mut s = HashSet::<i32>::new();
    for &num in &nums {
        if let None = s.get(&num) {
            if num > 0 {
                flag = true;
                s.insert(num);
                mx += num;
            }
        }
    }
    if !flag {
        mx = *nums.iter().max().unwrap();
    }
    mx
}
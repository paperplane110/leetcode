use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn longest_consecutive_sorted(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    let mut max_l = 1;
    let mut curr_l = 1;
    let mut prev = sorted_nums[0];

    for &num in sorted_nums[1..].iter() {
        if num == prev {
            continue;
        } else if num == prev + 1 {
            curr_l += 1;
            prev += 1;
        } else {
            max_l = max_l.max(curr_l);
            curr_l = 1;
            prev = num
        }
    }
    max_l.max(curr_l)
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<i32> = nums.into_iter().collect();

    let mut max_l = 0;
    for &n in num_set.iter() {
        if !num_set.contains(&(n - 1)) {
            let mut l = 1;
            let mut curr = n;
            while num_set.contains(&(curr + 1)) {
                l += 1;
                curr += 1;
            }
            max_l = max_l.max(l);
        }
    }
    max_l
}
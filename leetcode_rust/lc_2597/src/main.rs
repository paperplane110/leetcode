use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    println!("{}", beautiful_subsets(vec![2,4,6], 2));
    println!("{}", beautiful_subsets(vec![1], 1))
}

fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = -1;
    let mut cnt = HashMap::<i32, i32>::new();

    fn dfs(nums: &Vec<i32>, k: i32, i: usize, cnt: &mut HashMap<i32, i32>, ans: &mut i32) {
        if i == nums.len() {
            *ans += 1;
            return;
        }
        dfs(nums, k, i + 1, cnt, ans);
        let x = nums[i];
        if cnt.get(&(x - k)).copied().unwrap_or(0) == 0 && 
            cnt.get(&(x + k)).copied().unwrap_or(0) == 0 {
                *cnt.entry(x).or_insert(0) += 1;
                dfs(nums, k, i + 1, cnt, ans);
                *cnt.entry(x).or_insert(0) -= 1;
            }
    }
    dfs(&nums, k, 0, &mut cnt, &mut ans);
    ans
}
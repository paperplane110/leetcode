use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut ans: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut deque: VecDeque<usize> = VecDeque::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&back_idx) = deque.back() {
            if num >= nums[back_idx] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);
        if let Some(&front_idx) = deque.front() {
            if front_idx + k < i {
                deque.pop_front();
            }
        }
        if i >= k - 1 {
            ans.push(nums[*deque.front().unwrap()]);
        }
    }
    ans
}
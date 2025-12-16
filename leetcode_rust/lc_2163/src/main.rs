use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    println!("Hello, world!");
}

fn minimum_difference(nums: Vec<i32>) -> i64 {
    let m = nums.len();
    let n = m / 3;

    let mut s: i64 = 0;
    let mut pre: Vec<i64> = vec![0; m + 1];
    let mut pq = BinaryHeap::new();

    for i in 1..=2 * n {
        let x = nums[i - 1] as i64;
        s += x;
        pq.push(x);
        if pq.len() > n {
            if let Some(top) = pq.pop() {
                s -= top;
            }
        }
        pre[i] = s;
    }

    s = 0;
    let mut suf = vec![0; m + 1];
    let mut pq = BinaryHeap::new();
    for i in (n + 1..=m).rev() {
        let x = nums[i - 1] as i64;
        s += x;
        pq.push(Reverse(x));
        if pq.len() > n {
            if let Some(Reverse(top)) = pq.pop() {
                s -= top;
            }
        }
        suf[i] = s;
    }

    let mut ans = i64::MAX;
    for i in n..=2 * n {
        ans = ans.min(pre[i] - suf[i + 1]);
    }

    ans
}

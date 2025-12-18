fn main() {
    println!("Hello, world!");
}

fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut pre_max = vec![0; n];
    pre_max[0] = height[0];
    for i in 1..n {
        pre_max[i] = pre_max[i - 1].max(height[i]);
    }

    let mut suf_max = vec![0; n];
    suf_max[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        suf_max[i] = suf_max[i + 1].max(height[i]);
    }

    let mut ans = 0;
    for i in 0..n {
        ans += pre_max[i].min(suf_max[i]) - height[i];
    }
    ans
}

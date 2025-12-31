fn main() {
    println!("Hello, world!");
    let test = vec![1, 2, 3, 4];
    println!("{:?}", product_except_self(test));
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    ans.push(1);
    let mut tmp = 1;

    for i in 1..nums.len() {
        ans.push(ans[i-1] * nums[i-1]);
    }

    // traverse nums in reverse
    for i in (0..nums.len() - 1).rev() {
        println!("{}", tmp);
        tmp *= nums[i+1];
        ans[i] *= tmp;
    }

    ans
}
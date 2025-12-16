fn main() {
    let arr = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]];
    let ans = max_distance(&arr);
    println!("{}", ans);
}

fn max_distance(arr: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut mx = i32::MIN / 2;
    let mut mn = i32::MAX / 2;
    for a in arr {
        let a_min = a[0];
        let a_max = a[a.len() - 1];
        ans = ans.max(a_max - mn).max(mx - a_min);
        mn = mn.min(a_min);
        mx = mx.max(a_max);
    }
    ans
}

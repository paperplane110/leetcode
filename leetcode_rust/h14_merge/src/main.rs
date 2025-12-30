fn main() {
    println!("Hello, world!");
}

fn merge(interval: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut interval = interval;
    interval.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    let mut ans: Vec<Vec<i32>> = Vec::new();
    for p in interval {
        if let Some(last) = ans.last_mut() {
            if p[0] <= last[1] {
                last[1] = last[1].max(p[1]);
            } else {
                ans.push(p);
            }
        } else {
            ans.push(p);
        }
    }
    ans
}
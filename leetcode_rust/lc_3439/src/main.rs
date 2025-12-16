fn main() {
    println!("Hello, world!");
    max_free_time(21, 2, vec![18, 20], vec![20, 21]);
}

fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let mut free_times = Vec::new();

    free_times.push(start_time[0] - 0);
    for i in 1..start_time.len() {
        free_times.push(start_time[i] - end_time[i - 1]);
    }
    free_times.push(event_time - end_time[end_time.len() - 1]);
    println!("{:?}", free_times);

    let (mut ans, mut s) = (0, 0);
    for i in 0..free_times.len() {
        s += free_times[i];
        if i < k as usize {
            continue;
        }
        ans = ans.max(s);
        s -= free_times[i - k as usize];
    }
    println!("{}", ans);
    ans
}
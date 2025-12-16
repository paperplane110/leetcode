fn main() {
    println!("Hello, world!");
    let meeting = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    let days = 5;
    let res = count_days(days, meeting);
    println!("res: {}", res);
}


// meeting 已经是 owner 了
fn count_days(days: i32, meeting: Vec<Vec<i32>>) -> i32 {
    let mut end = 0;
    let mut free = 0;
    // 这里将 owner 给了可变 sorted_meetings
    let mut sorted_meetings = meeting;
    sorted_meetings.sort_by(|a, b| a[0].cmp(&b[0]));
    for meeting in sorted_meetings.into_iter() {
        let s = meeting[0];
        let e = meeting[1];
        if s > end {
            free += s - end - 1;
        }
        end = end.max(e);
    }
    free + days - end
}
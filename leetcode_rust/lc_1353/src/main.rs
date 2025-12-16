use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
    println!("{}", max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]));
}

fn max_events(events: Vec<Vec<i32>>) -> i32 {
    let mut mx = 0;
    for e in &events{
        mx = mx.max(e[1]);
    }

    let mut groups = vec![vec![]; (mx + 1) as usize];
    for e in &events{
        groups[e[0] as usize].push(e[1]);
    }

    let mut ans = 0;
    let mut h = BinaryHeap::<i32>::new();
    for (i, g) in groups.into_iter().enumerate() {
        while let Some(end_day) = h.peek() {
            if -end_day >= i as i32 {
                break;
            }
            h.pop();
        }
        for end_day in g {
            h.push(-end_day)
        }
        if let Some(_) = h.pop() {
            ans += 1
        }
    }
    ans
}
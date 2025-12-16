fn main() {
    println!("Hello, world!");
    let st = vec![1, 2, 3, 3];
    let et = vec![3, 4, 5, 6];
    let p = vec![50, 10, 40, 70];
    assert!(120 == job_scheduling(st, et, p));
}

fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs = start_time
        .into_iter()
        .zip(end_time.into_iter())
        .zip(profit.into_iter())
        .map(|((s, e), p)| (s, e, p))
        .collect::<Vec<(i32, i32, i32)>>();

    jobs.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let n = jobs.len();
    let mut f = vec![0; n + 1];
    for (i, &(s, _, p)) in jobs.iter().enumerate() {
        let j = jobs[..i]
            .partition_point(|job| job.1 < s);
        f[i + 1] = f[i].max(f[j + 1] + p);
    }
    f[n]
}

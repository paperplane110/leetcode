use std::collections::HashMap;

fn main() {
    let low_limit = 19;
    let high_limit = 28;
    let max_balls = count_balls(low_limit, high_limit);
    println!("max_balls: {}", max_balls);
}

fn ball2box(mut ball: i32) -> i32 {
    let mut res = 0;
    while ball > 0 {
        res += ball % 10;
        ball /= 10;
    }
    return res;
}

fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut box2ball: HashMap<i32, i32> = HashMap::new();
    for ball in low_limit..=high_limit {
        let box_num = ball2box(ball);
        box2ball
            .entry(box_num)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    // into_values 
    box2ball
        .into_values()
        .into_iter()
        .max()
        .unwrap()
}

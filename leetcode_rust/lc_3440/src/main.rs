fn main() {
    println!("Hello, world!");
}


fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    fn get(i: usize, start_time: &Vec<i32>, end_time: &Vec<i32>, event_time: i32) -> i32 {
        if i == 0 {
            return start_time[0]
        } else if i == start_time.len() {
            return event_time - end_time[i]
        } else {
            return end_time[i] - start_time[i]
        }
    }
    0
}
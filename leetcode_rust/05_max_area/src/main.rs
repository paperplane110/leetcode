fn main() {
    println!("Hello, world!");
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;
    let mut area = 0;
    while left < right {
        let w = (right - left) as i32;
        let h = height[left].min(height[right]);
        area = area.max(w * h);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    area
}

fn max_area_nested_for_loop(height: Vec<i32>) -> i32 {
    let mut area = 0;
    for i in 0..height.len()-1 {
        for j in i+1..height.len() {
            let w  = j - i;
            let h = height[i].min(height[j]);
            area = area.max(w as i32 * h)
        }
    }
    area
}
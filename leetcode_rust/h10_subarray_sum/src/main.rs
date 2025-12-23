use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut pre = 0;

    // sum to cnt
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, 1);

    for n in nums {
        pre += n;
        if let Some(&cnt) = map.get(&(pre - k)) {
            ans += cnt;
        }
        // map.entry(key) 回返回 Entry 类，有很多有用的链式方法来应对没有key的情况
        // 无 key，则可用 or_default, or_insert, or_insert_with 来插入值
        // 有 key，则可用 and_modify(|v| ...) 来更新值
        // 或者如下面这样，对 &mut i32 解引用
        *map.entry(pre).or_default() += 1;
    }
    ans
}
use std::{collections::HashMap, vec};

fn main() {
    println!("Hello, world!");
    let arr = vec!(12,33,4,56,22,2,34,33,22,12,34,56);
    let obj = RangeFreqQuery::new(arr);
    let res = obj.query(1, 5, 4);
    println!("res = {}", res);
}


struct RangeFreqQuery {
    pos: HashMap<i32, Vec<usize>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {

    fn new(arr: Vec<i32>) -> Self {
        let mut pos = HashMap::new();
        for (idx, &num) in arr.iter().enumerate() {
            pos.entry(num).or_insert(vec![]).push(idx);
        }
        Self {
            pos,
        }
    }
    
    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if let Some(a) = self.pos.get(&value) {
            let p = a.partition_point(|&x| x < left as usize);
            let q = a.partition_point(|&x| x <= right as usize);
            (q - p) as _
        } else {
            0
        }
    }
}
fn main() {
    println!("Hello, world!");
    let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
    let k = 3;
    println!("{}", has_increasing_subarrays(nums, k));
}

fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    let mut cnt = 1;
    let mut pre_cnt = 0;
    let mut ans = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i-1] {
            cnt += 1;
        } else {
            pre_cnt = cnt;
            cnt = 1;
        }
        ans = ans.max(pre_cnt.min(cnt));
        ans = ans.max(cnt / 2);
    }
    return ans >= k;
}

// fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
//     if nums.len() < 2 * k as usize {
//         return false;
//     }
//     for i in 0..=(nums.len() - 2 * k as usize) {
//         println!("{}", i);
//         let mut do_continue = false;
//         for j in i..(i + k as usize - 1) {
//             if &nums[j] >= &nums[j+1] {
//                 do_continue = true;
//                 break;
//             }
//         }
//         if do_continue { continue; }
//         for j in (i + k as usize)..(i + 2 * k as usize - 1) {
//             if &nums[j] >= &nums[j+1] {
//                 do_continue = true;
//                 break;
//             }
//         }
//         if do_continue {
//             continue;
//         } else {
//             return true;
//         }

//     }
//     return false;
// }

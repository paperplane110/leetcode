fn main() {
    println!("Hello, world!");
    let mut nums = vec![0,0,1,1,1,1,2,3,3];
    let l = remove_duplicates(&mut nums);
    println!("Length is {}, subsequence is {:?}", l, &nums[0..l as usize]);

    let mut nums_2 = vec![0, 0, 0, 1, 2, 3, 3, 3];
    let l = remove_duplicates_stack(&mut nums_2);
    println!("Length is {}, subsequence is {:?}", l, &nums_2[0..l as usize]);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2  {
        return nums.len() as i32;
    }
    let mut l = 0;
    let mut current = nums[0];
    let mut count = 0;
    for i in 0..nums.len() {
        if nums[i] == current {
            count += 1;
            if count <= 2 {
                l += 1;
            } else {
                continue;
            }
        } else {
            current = nums[i];
            count = 1;
            l += 1;
        }

        if i > (l - 1) {
            nums[l-1] = nums[i];
        }
    }
    return l as i32;
}

fn remove_duplicates_stack(nums: &mut Vec<i32>) -> i32 {
    let mut stack_size = 2;
    for i in 2..nums.len() {
        if nums[i] != nums[stack_size - 2] {
            nums[stack_size] = nums[i];
            stack_size += 1;
        }
    }
    stack_size.min(nums.len()) as _
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_remove_duplicates_empty_input() {
        let mut nums: Vec<i32> = vec![];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 0);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_remove_duplicates_at_most_two() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(nums[..5], vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_remove_duplicates_one_or_two_elements() {
        let mut nums1 = vec![1];
        let result1 = remove_duplicates(&mut nums1);
        assert_eq!(result1, 1);
        assert_eq!(nums1, vec![1]);

        let mut nums2 = vec![1, 2];
        let result2 = remove_duplicates(&mut nums2);
        assert_eq!(result2, 2);
        assert_eq!(nums2, vec![1, 2]);
    }
}
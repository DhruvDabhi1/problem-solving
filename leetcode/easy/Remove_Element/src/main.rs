use core::num;
use std::clone;

fn main() {
    println!("Hello, world!");
    let test = Solution;
    test.remove_element(&mut vec![3,2,2,3], 3);
}

struct Solution;

impl Solution {
    pub fn remove_element(&self, nums: &mut Vec<i32>, val: i32) -> i32 {
     *nums = nums.iter().cloned().filter(|&x| x != val).collect::<Vec<i32>>();
    nums.len() as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
    let mut solution = Solution;

    let mut nums = vec![3,2,2,3];
    assert_eq!(solution.remove_element(&mut nums, 3), 2);
    assert_eq!(nums[0..2], [2,2]);

    let mut nums = vec![0,1,2,2,3,0,4,2];
    assert_eq!(solution.remove_element(&mut nums, 2), 5);
    assert_eq!(nums[0..5].sort(), [0,0,1,3,4].sort());

    let mut nums = vec![1,1,1,1,1];
    assert_eq!(solution.remove_element(&mut nums, 1), 0);
    assert_eq!(nums.len(), 0);

    let mut nums = vec![1,2,3,4,5];
    assert_eq!(solution.remove_element(&mut nums, 6), 5);
    assert_eq!(nums[0..5], [1,2,3,4,5]);

    let mut nums = vec![1,1,2,2,3,3,4,4,5,5];
    assert_eq!(solution.remove_element(&mut nums, 2), 8);
    assert_eq!(nums[0..8].sort(), [1,1,3,3,4,4,5,5].sort());

    let mut nums = vec![1,1,1,1,2,2,2,2];
    assert_eq!(solution.remove_element(&mut nums, 1), 4);
    assert_eq!(nums[0..4], [2,2,2,2]);

    let mut nums = vec![1,2,3,4,5,6,7,8,9,10];
    assert_eq!(solution.remove_element(&mut nums, 10), 9);
    assert_eq!(nums[0..9], [1,2,3,4,5,6,7,8,9]);

    let mut nums = vec![10,10,10,10,10,10,10,10,10,10];
    assert_eq!(solution.remove_element(&mut nums, 10), 0);
    assert_eq!(nums.len(), 0);

    let mut nums = vec![1,2,3,4,5,6,7,8,9,10];
    assert_eq!(solution.remove_element(&mut nums, 1), 9);
    assert_eq!(nums[0..9], [2,3,4,5,6,7,8,9,10]);

    let mut nums = vec![1,1,1,1,2,2,2,2,3,3,3,3];
    assert_eq!(solution.remove_element(&mut nums, 3), 8);
    assert_eq!(nums[0..8].sort(), [1,1,1,1,2,2,2,2].sort());

    let mut nums = vec![1,2,3,4,5,6,7,8,9,10];
    assert_eq!(solution.remove_element(&mut nums, 5), 9);
    assert_eq!(nums[0..9].sort(), [1,2,3,4,6,7,8,9,10].sort());

    let mut nums = vec![10,9,8,7,6,5,4,3,2,1];
    assert_eq!(solution.remove_element(&mut nums, 5), 9);
    assert_eq!(nums[0..9].sort(), [1,2,3,4,6,7,8,9,10].sort());

    let mut nums = vec![1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2];
    assert_eq!(solution.remove_element(&mut nums, 1), 10);
    assert_eq!(nums[0..10], [2,2,2,2,2,2,2,2,2,2]);

    let mut nums = vec![1,2,3,4,5,6,7,8,9,10];
    assert_eq!(solution.remove_element(&mut nums, 11), 10);
    assert_eq!(nums[0..10], [1,2,3,4,5,6,7,8,9,10]);

    let mut nums = vec![];
    assert_eq!(solution.remove_element(&mut nums, 1), 0);
    assert_eq!(nums.len(), 0);
    }
}
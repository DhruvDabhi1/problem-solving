use core::num;
use std::ptr::null;


//[0,0,1,1,1,2,2,3,3,4]
fn main() {
    println!("Hello, world!");
    let m = Solution;
    m.remove_duplicates2(&mut vec![0,0,1,1,1,2,2,3,3,4]);
}

struct Solution;

impl Solution {
     pub fn remove_duplicates(self, nums: &mut Vec<i32>) -> i32 {
         //   let m =   nums.iter().filter(|&X| X%2 == 0).collect::<Vec<&i32>>();
         let mut support:Vec<i32> = vec![];
         support.push(1000);
    for i in 0..nums.len(){
        if i >= nums.len(){
            break;
        }
        for j in 0..support.len(){
            // println!("{i},{j}");
            if nums[i] == support[j]{
                nums.remove(i);
                
            }
            else {
                support.push(nums[i])
            }
        }
    }
    println!("{:?}",support);
        println!("{:?}",nums);
        2
    }
     pub fn remove_duplicates2(&self, nums: &mut Vec<i32>) -> i32 {
        
        nums.sort();
        let mut m:i32 = 1000000000;
        let mut j = 0;
        for  _ in 0..nums.len(){
             if j >= nums.len(){
            break;
        }
            if m == nums[j]{
                nums.remove(j);
                j = j -1;
            }
            else {
                m=nums[j]
            }
            j+=1;
        }
        println!("{:?}",nums);
        nums.len() as i32
     }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates2() {
    let mut solution = Solution;

    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    assert_eq!(solution.remove_duplicates2(&mut nums), 5);
    assert_eq!(nums, vec![0,1,2,3,4]);

    let mut nums = vec![1,1,2];
    assert_eq!(solution.remove_duplicates2(&mut nums), 2);
    assert_eq!(nums, vec![1,2]);

    let mut nums = vec![1,1,1,1,1];
    assert_eq!(solution.remove_duplicates2(&mut nums), 1);
    assert_eq!(nums, vec![1]);

    let mut nums = vec![1,2,3,4,5];
    assert_eq!(solution.remove_duplicates2(&mut nums), 5);
    assert_eq!(nums, vec![1,2,3,4,5]);

    let mut nums = vec![];
    assert_eq!(solution.remove_duplicates2(&mut nums), 0);
    assert_eq!(nums, vec![]);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_remove_duplicates2() {
        let mut solution = Solution;
    
        let mut nums = vec![1,1,1,1,2,2,2,3,3,3,4,4,4];
        assert_eq!(solution.remove_duplicates2(&mut nums), 4);
        assert_eq!(nums[0..4], [1,2,3,4]);
    
        let mut nums = vec![5,5,5,5,5];
        assert_eq!(solution.remove_duplicates2(&mut nums), 1);
        assert_eq!(nums[0], 5);
    
        let mut nums = vec![6,7,8,9,10];
        assert_eq!(solution.remove_duplicates2(&mut nums), 5);
        assert_eq!(nums[0..5], [6,7,8,9,10]);
    
        let mut nums = vec![11,11,12,12,13,13,14,14,15,15];
        assert_eq!(solution.remove_duplicates2(&mut nums), 5);
        assert_eq!(nums[0..5], [11,12,13,14,15]);
    
        let mut nums = vec![16,17,18,19,20,20,20,20,20,20];
        assert_eq!(solution.remove_duplicates2(&mut nums), 5);
        assert_eq!(nums[0..5], [16,17,18,19,20]);
        }
    }
}
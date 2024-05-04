use std::{collections::HashMap, fmt::DebugMap, iter};

fn main() {
    println!("Hello, world!");
    let sol = Solution;
    // sol.two_sum(vec![3,3], 6);
    let value = sol.two_sum_(vec![1,2,3,4],7);
    println!("{:?}",value);
}

struct Solution;

impl Solution {
    pub fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let dumy = nums.clone();
        let mut value:Vec<i32> = vec![];
        //  let value = nums.iter().filter(|x|Some(dumy.iter().filter(|y|  **y + **x == target).cloned().next()).is_some()).clone().next();
        //  println!("{:?}",value);
        for i in 0..nums.len(){
            let mut m = dumy.len() - 1;
            for j in dumy.iter().rev(){
                if nums[i] + j == target && i != m{
                    value.push(i as i32);
                    value.push(m as i32);
                    println!("{:?}",value);
                    return value;
                }
                m -= 1;
            }
        }
        value
    }

      pub fn two_sum_(&self,nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h_map = HashMap::new();

        for (i, y) in nums.iter().enumerate() {
            let x = target - y;

            if let Some(&j) = h_map.get(&x) {
                return vec![j as i32, i as i32];
            }

            h_map.insert(y, i);
        }
        
        vec![0, 0]
    }
}
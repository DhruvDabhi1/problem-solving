use core::num;

fn main() {
    println!("Hello, world!");
    let data = Solution;
    // let data = data.final_rod(vec![8,9,9,4,10,5,6,9,7,9]);
    // println!("{:?}", data);
    let data = data.gpt(vec![3,50,9,30,40]);
    println!("{data}")
}

struct Solution;

impl Solution {
    pub fn rob(self, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut sum2 = 0;
        if nums.len() == 2 {
            if nums[0] > nums[1] {
                nums[0]
            } else {
                nums[1]
            }
        //  sum
        } else {
            for i in 0..nums.len() {
                if i == 0 || i % 2 == 0 {
                    sum += nums[i]
                }
                if i % 2 != 0 {
                    sum2 += nums[i]
                }
                println!("{}", i)
            }
            if sum > sum2 {
                sum
            } else {
                sum2
            }
        }
    }

    pub fn final_rod(self, nums: Vec<i32>) -> i32 {
        let mut sum = vec![];
        let mut sum2 = vec![];
        for i in 0..nums.len() {
            if i == 0 || i % 2 == 0 {
                sum.push(nums[i])
            }
            if i % 2 != 0 {
                sum2.push(nums[i])
            }
        }

        let mut final_sum = vec![];
        let mut final_sum2 = vec![];
        let final_sum3: i32 = sum.iter().sum();
        let final_sum4: i32 = sum2.iter().sum();
        for i in 0..sum.len() {
            final_sum.push(sum[i] + sum_of_that(i as i32, -1, sum2.clone()));
        }
        for i in 0..sum2.len() {
            final_sum2.push(sum2[i] + sum_of_that(i as i32, 1, sum.clone()));
        }
        pub fn sum_of_that(current_index: i32, updated_index: i32, data: Vec<i32>) -> i32 {
            let mut m = 0;
            let mut sum = 0;
            for i in 0..data.len() {
                if m != current_index && m != current_index + updated_index {
                    sum += data[i]
                }
                m += 1;
            }
            sum
        }
        pub fn final_sum_of_that(index1: Vec<i32>, index2: Vec<i32>) -> i32 {
            let mut data1: Vec<i32> = vec![];
            for j in 0..index1.len() {
                let mut sum = 0;
                'a: for i in 0..index1.len() {
                    if i == index1.len() - 1 {
                        // sum += index2[i];
                        break 'a;
                    }
                    sum += index1[i] + index2[i + 1];
                }
                data1.push(sum)
            }
            data1.iter().max().unwrap().clone()
        }
        let final_sum = final_sum.iter().max().unwrap().clone();
        let final_sum2 = final_sum2.iter().max().unwrap().clone();
        let final_sum5 = final_sum_of_that(sum, sum2);
        println!(
            "{},{},{},{},{}",
            final_sum, final_sum2, final_sum3, final_sum4, final_sum5
        );
        if final_sum > final_sum2
            && final_sum > final_sum3
            && final_sum > final_sum4
            && final_sum > final_sum5
        {
            final_sum
        } else {
            if final_sum2 > final_sum3 && final_sum2 > final_sum4 && final_sum2 > final_sum5 {
                final_sum2
            } else {
                if final_sum3 > final_sum4 && final_sum3 > final_sum5 {
                    final_sum3
                } else {
                    if final_sum4 > final_sum5 {
                        final_sum4
                    } else {
                        final_sum5
                    }
                }
            }
        }
    }

    fn sol(self, nums: Vec<i32>) -> i32 {
        nums.iter().fold((0, 0), |(a, b), x| (b, b.max(a + x))).1
    }
    fn sol2(self, nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];
        for (i, num) in nums.iter().enumerate() {
            println!("{i},{num}");
            dp[i + 2] = dp[i + 2].max(dp[i] + num);
            dp[i + 1] = dp[i + 1].max(dp[i]);
            println!("{:?}", dp)
        }
        *dp.iter().max().unwrap()
    }
    fn gpt(self, nums: Vec<i32>) -> i32 {
        let mut max_not_robbed = 0;
        let mut max_robbed = 0;

        for &house in nums.iter() {
            println!("{house}");
            let rob = house + max_not_robbed;
            let not_rob = std::cmp::max(max_robbed, max_not_robbed);

            max_robbed = rob;
            max_not_robbed = not_rob;
        }

        std::cmp::max(max_robbed, max_not_robbed)
    }
}

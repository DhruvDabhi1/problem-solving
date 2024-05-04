
fn main() {
    println!("Hello, world!");
    let solution = Solution;
    solution.search_insert(vec![1,3,5,6], 5);
}

struct Solution;
impl Solution {
    pub fn search_insert(&self,nums: Vec<i32>, target: i32) -> i32 {
        let mut f_index:i32=100;
       for (index,values) in nums.iter().enumerate(){
        if *values == target{
            f_index = index as i32;
            println!("{}",f_index)
        }
       }
        return f_index;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        let solution = Solution;

        assert_eq!(solution.search_insert(vec![1,3,5,6], 5), 2);
        assert_eq!(solution.search_insert(vec![1,3,5,6], 2), 1);
        assert_eq!(solution.search_insert(vec![1,3,5,6], 7), 4);
        assert_eq!(solution.search_insert(vec![1,3,5,6], 0), 0);
        assert_eq!(solution.search_insert(vec![1], 0), 0);
        assert_eq!(solution.search_insert(vec![1], 2), 1);
        assert_eq!(solution.search_insert(vec![1,3], 2), 1);
        assert_eq!(solution.search_insert(vec![1,3,5], 4), 2);
        assert_eq!(solution.search_insert(vec![1,3,5,7], 6), 3);
        assert_eq!(solution.search_insert(vec![1,3,5,7,9], 8), 4);
        assert_eq!(solution.search_insert(vec![1,3,5,7,9,11], 10), 5);
        assert_eq!(solution.search_insert(vec![1,3,5,7,9,11,13], 12), 6);
        assert_eq!(solution.search_insert(vec![1,3,5,7,9,11,13,15], 14), 7);
        assert_eq!(solution.search_insert(vec![1,3,5,7,9,11,13,15,17], 16), 8);
        assert_eq!(solution.search_insert(vec![1,3,5,7,9,11,13,15,17,19], 18), 9);
    }
}
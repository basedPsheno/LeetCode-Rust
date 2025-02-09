use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut bad_pairs = 0;
        let mut map = HashMap::new();

        for i in 0..n {
            bad_pairs += i as i64;
            if let Some(&count) = map.get(&(nums[i] - i as i32)) {
                bad_pairs -= count;
            }
            *map.entry(nums[i] - i as i32).or_insert(0) += 1;
        }

        bad_pairs
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn count_bad_pairs_example_1() {
        assert_eq!(Solution::count_bad_pairs(vec![4,1,3,3]), 5);
    }

    #[test]
    fn count_bad_pairs_example_2() {
        assert_eq!(Solution::count_bad_pairs(vec![1,2,3,4,5]), 0);
    }
}

fn main() {}
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        let mut ans = 0;

        for i in 0..nums.len() {
            for j in (i+1)..nums.len() {
                counter.entry(nums[i] * nums[j])
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }

        for (_, &val) in counter.iter() {
            if val > 1 {
                ans += 4 * val * (val - 1);
            }
        }

        ans
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn tuple_same_product_example_1() {
        assert_eq!(Solution::tuple_same_product(vec![2,3,4,6]), 8);
    }

    #[test]
    fn tuple_same_product_example_2() {
        assert_eq!(Solution::tuple_same_product(vec![1,2,4,5,10]), 16);
    }

    #[test]
    fn tuple_same_product_example_3() {
        assert_eq!(Solution::tuple_same_product(vec![2,3,4,6,8,12]), 40);
    }
}

fn main() {}
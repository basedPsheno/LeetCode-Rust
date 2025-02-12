use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = -1;
        let mut digits_sums: HashMap<i32, (i32, i32)> = HashMap::new();

        for i in 0..n {
            let mut sum = 0;
            let mut num = nums[i];

            while num > 0 {
                sum += num % 10;
                num /= 10;
            }

            let num = nums[i];
            if digits_sums.contains_key(&sum) {
                let (a, b) = digits_sums[&sum];
                let mut_digits_sum = digits_sums.get_mut(&sum).unwrap();
                if b > 0 {
                    if num > b {
                        mut_digits_sum.1 = num;
                        mut_digits_sum.0 = b;
                    } else if num > a {
                        mut_digits_sum.0 = num;
                    }
                } else {
                    if num < a {
                        mut_digits_sum.0 = num;
                        mut_digits_sum.1 = a;
                    } else {
                        mut_digits_sum.1 = num;
                    }
                }

                if mut_digits_sum.0 + mut_digits_sum.1 > ans {
                    ans = mut_digits_sum.0 + mut_digits_sum.1;
                }
            } else {
                digits_sums.insert(sum, (num, 0));
            }
        }

        ans
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn maximum_sum_example_1() {
        assert_eq!(Solution::maximum_sum(vec![18,43,36,13,7]), 54);
    }

    #[test]
    fn maximum_sum_example_2() {
        assert_eq!(Solution::maximum_sum(vec![10,12,19,14]), -1);
    }
}

fn main() {}
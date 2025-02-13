use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut btree = BTreeMap::new();
        let mut ans = 0;

        for num in nums {
            *btree.entry(num as i64).or_insert(0) += 1;
        }

        println!("{:?}", btree);

        while *btree.first_key_value().unwrap().0 < k as i64 {
            println!("{:?}", btree);
            let (mut min, mut max) = (-1, -1);
            if let Some((x, count)) = btree.pop_first() {
                if count > 1 {
                    btree.insert(x, count - 1);
                }
                min = x;
            }
            if let Some((y, count)) = btree.pop_first() {
                if count > 1 {
                    btree.insert(y, count - 1);
                }
                max = y;
            }

            btree.entry(min * 2 + max).and_modify(|v| *v += 1).or_insert(1);

            ans += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn min_operations_example_1() {
        assert_eq!(Solution::min_operations(vec![2,11,10,1,3], 10), 2);
    }

    #[test]
    fn min_operations_example_2() {
        assert_eq!(Solution::min_operations(vec![1,1,2,4,9], 20), 4);
    }
}

fn main() {}
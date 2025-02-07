use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball_color = HashMap::new();
        let mut color_count = HashMap::new();
        let mut ans = Vec::with_capacity(queries.len());

        for i in queries {
            let x = i[0];
            let y = i[1];

            if let Some(&old_color) = ball_color.get(&x) {
                if let Some(count) = color_count.get_mut(&old_color) {
                    *count -= 1;
                    if *count == 0 {
                        color_count.remove(&old_color);
                    }
                }
            }

            ball_color.insert(x, y);

            *color_count.entry(y).or_insert(0) += 1;

            ans.push(color_count.len() as i32);
        }

        ans
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn query_results_example_1() {
        assert_eq!(Solution::query_results(4, vec![vec![1,4], vec![2,5], vec![1,3], vec![3,4]]), vec![1,2,2,3]);
    }

    #[test]
    fn query_results_example_2() {
        assert_eq!(Solution::query_results(4, vec![vec![0,1], vec![1,2], vec![2,2], vec![3,4], vec![4,5]]), vec![1,2,2,3,4]);
    }
}

fn main() {}
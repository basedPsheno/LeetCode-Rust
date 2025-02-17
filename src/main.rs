struct Solution;

impl Solution {
    fn num_tile_possibilities(tiles: String) -> i32 {
        let mut freq = [0; 26];
        for c in tiles.chars() {
            freq[c as usize - 'A' as usize] += 1;
        }
        
        fn backtrack(freq: &mut [i32; 26], total: &mut i32) {
            for i in 0..26 {
                if freq[i] > 0 {
                    freq[i] -= 1;
                    *total += 1;
                    
                    let mut new_freq = *freq;
                    backtrack(&mut new_freq, total);
                    
                    freq[i] += 1;
                }
            }
        }
        
        let mut total = 0;
        backtrack(&mut freq, &mut total);
        total
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn num_tile_possibilities_example_1() {
        assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
    }

    #[test]
    fn num_tile_possibilities_example_2() {
        assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
    }

    #[test]
    fn num_tile_possibilities_example_3() {
        assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
    }
}

fn main() {}
struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let n = s.len();
        let m = part.len();
        let part_chars: Vec<char> = part.chars().collect();
        let mut stack = Vec::with_capacity(n);

        for c in s.chars() {
            stack.push(c);
            
            if stack.len() >= m && stack[stack.len() - m..] == part_chars {
                stack.truncate(stack.len() - m);
            }
        }

        stack.into_iter().collect::<String>()
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn remove_occurrences_example_1() {
        assert_eq!(Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()), "dab".to_string());
    }

    #[test]
    fn remove_occurrences_example_2() {
        assert_eq!(Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()), "ab".to_string());
    }
}

fn main() {}
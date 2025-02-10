struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut char_pointer = Vec::new();
        let n = s.len();
        let mut check = vec![true; n];
        let mut ans = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_digit() {
                if let Some(ch) = char_pointer.pop() {
                    check[ch] = false;
                }
                check[i] = false;
            }

            if c.is_ascii_alphabetic() {
                char_pointer.push(i);
            }

        }

        for (i, c) in s.chars().enumerate() {
            if check[i] {
                ans.push(c);
            }
        }

        ans.into_iter().collect::<String>()
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn clear_digits_example_1() {
        assert_eq!(Solution::clear_digits("abc".to_string()), "abc".to_string());
    }

    #[test]
    fn clear_digits_example_2() {
        assert_eq!(Solution::clear_digits("cb34".to_string()), "".to_string());
    }
}

fn main() {}
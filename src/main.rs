struct Solution;

impl Solution{
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        // по условию строки содержать только из строчных английских букв
        let (mut buffer_char1, mut buffer_char2) = ('-', '-');
        let mut diff_counter = 0;

        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                diff_counter += 1;
                if diff_counter == 1 {
                    buffer_char1 = c1;
                    buffer_char2 = c2;
                }
                if diff_counter == 2 {
                    if buffer_char1 != c2 || buffer_char2 != c1 {
                        return false;
                    }
                }
                if diff_counter > 2 {
                    return false;
                }
            }
        }

        if diff_counter == 1 {
            return false;
        }
        true
    }
}

fn main() {
    // к следующим задачам сделаю полноценный тест-модуль
    let s1 = "bank".to_string();
    let s2 = "kanb".to_string();

    let sol = Solution::are_almost_equal(s1, s2);
    println!("{}", sol);
}
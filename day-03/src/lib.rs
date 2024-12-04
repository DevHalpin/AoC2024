pub fn part1(input: &str) -> String {
    let mut sum = 0;
    let mut i = 0;

    let chars: Vec<char> = input.chars().collect();
    while i < chars.len() {
        if chars[i..].starts_with(&['m', 'u', 'l', '(']) {
            let mut start = i + 4;
            let mut num1 = String::new();
            let mut num2 = String::new();

            while start < chars.len() && chars[start].is_digit(10) {
                num1.push(chars[start]);
                start += 1;
            }

            if start < chars.len() && chars[start] == ',' {
                start += 1;
            } else {
                i = start;
                continue;
            }

            while start < chars.len() && chars[start].is_digit(10) {
                num2.push(chars[start]);
                start += 1;
            }

            if start < chars.len() && chars[start] == ')' {
                if let (Ok(num1), Ok(num2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                    sum += num1 * num2;
                }
            }

            i = start + 1;
        } else {
            i += 1;
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut sum2 = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut enabled = true;

    while i < chars.len() {
        if chars[i..].starts_with(&['d', 'o', '(', ')']) {
            enabled = true;
            i += 4;
        } else if chars[i..].starts_with(&['d', 'o', 'n', '\'', 't', '(', ')'])
        {
            enabled = false;
            i += 7;
        } else if chars[i..].starts_with(&['m', 'u', 'l', '(']) {
            if enabled {
                let mut start = i + 4;
                let mut num1 = String::new();
                let mut num2 = String::new();

                while start < chars.len() && chars[start].is_digit(10) {
                    num1.push(chars[start]);
                    start += 1;
                }

                if start < chars.len() && chars[start] == ',' {
                    start += 1;
                } else {
                    i = start;
                    continue;
                }

                while start < chars.len() && chars[start].is_digit(10) {
                    num2.push(chars[start]);
                    start += 1;
                }

                if start < chars.len() && chars[start] == ')' {
                    if let (Ok(num1), Ok(num2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                        sum2 += num1 * num2;
                    }
                }

                i = start + 1;
            } else {
                i += 4;
            }
        } else {
            i += 1;
        }
    }
    sum2.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "161");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT2);
        assert_eq!(result, "48");
    }
}

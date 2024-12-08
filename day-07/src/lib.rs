use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
  let mut parsed_input = Vec::new();
  for line in input.lines() {
    let parts: Vec<&str> = line.split(':').collect();
    let target: i64 = parts[0].trim().parse().unwrap();
    let nums: Vec<i64> = parts[1].trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    parsed_input.push((target, nums));
  }
  parsed_input 
}

fn is_fixable_equation(target: i64, parts: &[i64], allow_concatenation: bool) -> i64 {
  let mut potential_targets = HashSet::new();
    potential_targets.insert(target);

    for &n in parts.iter().rev() {
        let mut new_potential_targets = HashSet::new();

        for &t in &potential_targets {
            // Check addition "undo": if t - n is positive, it is a valid possibility
            let prev = t - n;
            if prev >= 0 {
                new_potential_targets.insert(prev);
            }

            // Check multiplication "undo": t would have to be a multiple of n if this is a valid possibility so we check if t is divisible by n with no remainder
            if n != 0 && t >= n && t % n == 0 {
                new_potential_targets.insert(t / n);
            }

            // Check concatenation "undo": if the last digit of t is the same as the last digit of n, we can remove the last digit of t and repeat the process as long as n is not 0
            if allow_concatenation {
                let mut t_temp = t;
                let mut n2 = n;
                let mut good = true;
                while n2 > 0 && good {
                    if n2 % 10 == t_temp % 10 {
                        t_temp /= 10;
                        n2 /= 10;
                    } else {
                        good = false;
                    }
                }

                if good {
                    new_potential_targets.insert(t_temp);
                }
            }
        }

        potential_targets = new_potential_targets;
        if potential_targets.is_empty() {
            // If at any point no potential targets remain, we can stop early.
            return 0
        }
    }
    // if our potential targets contain 0, we have figured out a way to get to the target
    if potential_targets.contains(&0) {
      return target
    } else {
      return 0
    }
}


pub fn part1(input: &str) -> String {
  let start = std::time::Instant::now();
  let parsed_input = parse_input(input);

  let mut count = 0;
  for (target, parts) in parsed_input {
    let result = is_fixable_equation(target, &parts, false);
    count += result;
  }
  let elapsed = start.elapsed();
  println!("Part 1 Time: {:?}", elapsed);
  count.to_string()
}

pub fn part2(input: &str) -> String {
  let start = std::time::Instant::now();
  let parsed_input = parse_input(input);
  
  let mut count = 0;
  for (target, parts) in parsed_input {
    let result = is_fixable_equation(target, &parts, true);
    count += result;
  }
  let elapsed = start.elapsed();
  println!("Part 2 Time: {:?}", elapsed);
  count.to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

  #[test]
  fn part1_works() {
      let result = part1(INPUT);
      assert_eq!(result, "3749");
  }

  #[test]
  fn part2_works() {
      let result = part2(INPUT);
      assert_eq!(result, "11387");
  }
}
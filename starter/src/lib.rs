pub fn part1(input: &str) -> String {
  "nothing".to_string()
}

pub fn part2(input: &str) -> String {
  "nothing".to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

  #[test]
  fn part1_works() {
      let result = part1(INPUT);
      assert_eq!(result, "157");
  }

  #[test]
  fn part2_works() {
      let result = part2(INPUT);
      assert_eq!(result, "70");
  }
}
use std::collections::HashMap;
pub fn part1(input: &str) -> String {
  let mut list1 = Vec::new();
  let mut list2 = Vec::new();
  for line in input.lines() {
    let parts: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    if parts.len() == 2 {
      list1.push(parts[0]);
      list2.push(parts[1]);
    }
  }

  list1.sort_unstable();
  list2.sort_unstable();

  let total_distance = list1.iter().zip(list2.iter()).fold(0, |acc, (a, b)| acc + (a - b).abs());

  // println!("{:?}", list1);
  // println!("{:?}", list2);
  // println!("{:?}", total_distance);
  total_distance.to_string()
}

pub fn part2(input: &str) -> String {
  let mut list1 = Vec::new();
  let mut list2 = Vec::new();
  let mut occurences = HashMap::new();
  for line in input.lines() {
    let parts: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    if parts.len() == 2 {
      list1.push(parts[0]);
      list2.push(parts[1]);
    }
  }
  // Count occurrences of each number in list2
  for num in list2 {
    *occurences.entry(num).or_insert(0) += 1;
  }

  let mut score = 0;
  for num in list1 {
    let occurences_of_num = occurences.get(&num).unwrap_or(&0);
    score += num * occurences_of_num;
  }
  score.to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

  #[test]
  fn part1_works() {
      let result = part1(INPUT);
      assert_eq!(result, "11");
  }

  #[test]
  fn part2_works() {
      let result = part2(INPUT);
      assert_eq!(result, "31");
  }
}
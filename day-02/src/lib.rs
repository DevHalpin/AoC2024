fn is_safe(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for i in 0..report.len() - 1 {
      let abs_diff = (report[i] - report[i + 1]).abs();

      if abs_diff < 1 || abs_diff > 3 {
        return false;
      }

      if report[i] > report[i + 1] {
        increasing = false;
      } else if report[i] < report[i + 1] {
        decreasing = false;
      }
    }

    increasing || decreasing
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
  if is_safe(report) {
    return true;
  }

  for i in 0..report.len() {
    let mut modded_report = report.to_vec();
    modded_report.remove(i);
    if is_safe(&modded_report) {
      return true;
    }
  }
  false
}

pub fn part1(input: &str) -> String {
  let output = input.lines()
  .map(|line| {
    line.split_whitespace().filter_map(|s| s.parse().ok()).collect::<Vec<i32>>() 
  }).filter(|report| is_safe(&report))
  .count();
  
  output.to_string()
}

pub fn part2(input: &str) -> String {
  let output = input.lines()
  .map(|line| {
    line.split_whitespace().filter_map(|s| s.parse().ok()).collect::<Vec<i32>>() 
  }).filter(|report| is_safe_with_dampener(&report))
  .count();
  
  output.to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

  #[test]
  fn part1_works() {
      let result = part1(INPUT);
      assert_eq!(result, "2");
  }

  #[test]
  fn part2_works() {
      let result = part2(INPUT);
      assert_eq!(result, "4");
  }
}
pub fn part1(input: &str) -> String {
  let directions: [(isize, isize); 8] = [
    (0,1),  //Right
    (0,-1), //Left
    (1,0), // Down
    (-1,0), // Up
    (1,1), // Down Right
    (-1,1), // Up Right
    (1,-1), // Down Left
    (-1,-1), // Up Left
  ];
  let word = ['X', 'M', 'A', 'S'];
  let mut count = 0;
  let grid: Vec<Vec<char>> = input.lines().map(|line| {
    line.chars().collect::<Vec<char>>()
  }).collect();
  let rows = grid.len();
  let cols = grid[0].len();
  
  for row in 0..rows {
    for col in 0..cols {
      for &(dr, dc) in &directions {
        let mut found = true;
        for i in 0..word.len() {
          let nr = row as isize + (dr * i as isize) ;
          let nc = col as isize + (dc * i as isize);
          if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
            found = false;
            break;
          }
          if grid[nr as usize][nc as usize] != word[i] {
            found = false;
            break;
          }
        }
        if found {
          count += 1;
        }
      }
    }
  }
  count.to_string()
}

pub fn part2(input: &str) -> String {
  let mut count = 0;

  let grid: Vec<Vec<char>> = input.lines().map(|line| {
      line.chars().collect::<Vec<char>>()
  }).collect();

  let rows = grid.len();
  let cols = grid[0].len();

  for row in 1..rows - 1 {
    for col in 1..cols - 1 {
      if grid[row][col] == 'A' {
        let top_left = grid[row - 1][col - 1];
        let top_right = grid[row - 1][col + 1];
        let bottom_left = grid[row + 1][col - 1];
        let bottom_right = grid[row + 1][col + 1];

        // Check the conditions for an X-MAS pattern
        if (top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M') &&
           (bottom_left == 'M' && top_right == 'S' || bottom_left == 'S' && top_right == 'M') {
            count += 1;
        }
      }      
    }
  }

  count.to_string()
}


#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

  #[test]
  fn part1_works() {
      let result = part1(INPUT);
      assert_eq!(result, "18");
  }

  #[test]
  fn part2_works() {
      let result = part2(INPUT);
      assert_eq!(result, "9");
  }
}
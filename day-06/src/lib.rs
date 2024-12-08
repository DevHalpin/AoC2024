use std::{collections::HashSet, time::Instant};

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize), (isize, isize)) {
    let mut map_data = Vec::new();
    let mut guard_pos = (0, 0);
    let mut guard_dir = (0, 0);
    
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    guard_pos = (x, y);
                    guard_dir = (0, -1);
                    row.push('.');
                }
                '>' => {
                    guard_pos = (x, y);
                    guard_dir = (1, 0);
                    row.push('.');
                }
                'v' => {
                    guard_pos = (x, y);
                    guard_dir = (0, 1);
                    row.push('.');
                }
                '<' => {
                    guard_pos = (x, y);
                    guard_dir = (-1, 0);
                    row.push('.');
                }
                _ => row.push(c),
            }
        }
        map_data.push(row);
    }
    
    (map_data, guard_pos, guard_dir)
}

fn turn_right(dir: (isize, isize)) -> (isize, isize) {
  match dir {
      (0, -1) => (1, 0),  // ^ turns right to ->
      (0, 1)  => (-1, 0), // v turns right to <-
      (-1, 0) => (0, -1), // <- turns right to ^
      (1, 0)  => (0, 1),  // -> turns right to v
      _ => dir,
  }
}


fn simulate_guard_obstructions(
    grid: &Vec<Vec<char>>, 
    mut pos: (usize, usize), 
    mut dir: (isize, isize),
    obstruction: (usize, usize)
) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    
    while pos.0 < cols && pos.1 < rows {
      if visited.contains(&(pos, dir)) {
        return true;
      }
      visited.insert((pos, dir));
      
      // Calculate the next position
      let next_pos = (
          (pos.0 as isize + dir.0) as usize,
          (pos.1 as isize + dir.1) as usize,
      );
      
      // Check if the next position is within bounds and blocked
      if next_pos.1 < rows
          && next_pos.0 < cols 
          && (grid[next_pos.1][next_pos.0] == '#' || next_pos == obstruction)
      {
          // Turn right if there's an obstacle
          dir = turn_right(dir);
      } else {
          pos = next_pos;
      }
    }
    return false
}

fn simulate_guard(
    grid: &Vec<Vec<char>>, 
    mut pos: (usize, usize), 
    mut dir: (isize, isize)
) -> (usize, HashSet<(usize, usize)>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    
    while pos.0 < cols && pos.1 < rows {
        visited.insert(pos);
        
        // Calculate the next position
        let next_pos = (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        );
        
        // Check if the next position is within bounds and blocked
        if next_pos.1 < rows
            && next_pos.0 < cols
            && grid[next_pos.1][next_pos.0] == '#'
        {
            // Turn right if there's an obstacle
            dir = turn_right(dir);
        } else {
            // Move forward
            if next_pos.0 >= cols || next_pos.1 >= rows {
                break; // Exit if out of bounds
            }
            pos = next_pos;
        }
    }
    
    (visited.len(), visited)
}


pub fn part1(input: &str) -> String {
  let (map_data, guard_pos, guard_dir) = parse_input(input);
  let distinct_positions = simulate_guard(&map_data, guard_pos, guard_dir);
  distinct_positions.0.to_string()
}

pub fn part2(input: &str) -> String {
  let start = Instant::now();
  let mut count = 0;
  let (map_data, guard_pos, guard_dir) = parse_input(input);
  let distinct_positions = simulate_guard(&map_data,guard_pos, guard_dir);
  for obstruction in distinct_positions.1 {
    let obstructed = simulate_guard_obstructions(&map_data, guard_pos, guard_dir, obstruction);
    if obstructed {
      count += 1;
    }
  }
  let duration = start.elapsed();
  println!("Time elapsed in expensive_function() is: {:?}", duration);
  count.to_string()

}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

  #[test]
  fn part1_works() {
      let result = part1(INPUT);
      assert_eq!(result, "41");
  }

  #[test]
  fn part2_works() {
      let result = part2(INPUT);
      assert_eq!(result, "6");
  }
}
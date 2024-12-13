use std::collections::{HashMap, HashSet};
// HashMap is used to group antennas by their frequency
// HashSet is used to store unique antinode positions

// Define a struct to represent a position on the map with x and y coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize, 
    y: isize, 
}

// Define a struct to represent a line in form: Ax + By + C = 0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Line {
    a: isize, 
    b: isize, 
    c: isize, 
}

/// Computes the greatest common divisor using the Euclidean algorithm.
/// This function is used to normalize the coefficients of the line equation to make finding unique lines easier.
fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Line {
    /// This ensures that lines that are the same have identical representations.  Just... all the math...
    fn from_positions(p1: &Position, p2: &Position) -> Self {
        let a = p2.y - p1.y; 
        let b = p1.x - p2.x; 
        let c = p2.x * p1.y - p1.x * p2.y; 

        // Normalize the coefficients by dividing by their greatest common divisor
        let gcd_ab = gcd(a.abs(), b.abs());
        let gcd_all = gcd(gcd_ab, c.abs());

        let a_norm = a / gcd_all;
        let b_norm = b / gcd_all;
        let c_norm = c / gcd_all;

        // Ensure that the leading coefficient 'a' is non-negative for uniqueness
        if a_norm < 0 || (a_norm == 0 && b_norm < 0) {
            Line {
                a: -a_norm,
                b: -b_norm,
                c: -c_norm,
            }
        } else {
            Line {
                a: a_norm,
                b: b_norm,
                c: c_norm,
            }
        }
    }
}

pub fn part1(input: &str) -> String {
    // Parse the input into a vector of string slices, each representing a row of the map
    let map: Vec<&str> = input.lines().collect();

    // Determine the height and width of the map
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    // Create a HashMap to group antennas by their frequency
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    // Iterate over each row and column to identify antennas and their positions
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell != '.' { // '.' represents an empty space; other characters are antennas
                antennas.entry(cell).or_insert_with(Vec::new)
                    .push(Position { x: x as isize, y: y as isize });
            }
        }
    }

    // Create a HashSet to store unique antinode positions
    let mut antinodes: HashSet<Position> = HashSet::new();

    // Iterate over each frequency and its corresponding list of antennas
    for (_freq, antenna_list) in &antennas {
        let n = antenna_list.len();
        if n < 2 {
            continue; // Skip frequencies with fewer than two antennas, as they cannot form antinodes
        }

        // Iterate over all unique pairs of antennas for the current frequency
        for i in 0..n {
            for j in (i + 1)..n {
                let a = antenna_list[i]; 
                let b = antenna_list[j]; 

                // Calculate two potential antinode positions based on the pair
                // P = 2B - A ensures that B is twice as far from P as A is
                let p = Position {
                    x: 2 * b.x - a.x,
                    y: 2 * b.y - a.y,
                };
                // Q = 2A - B ensures that A is twice as far from Q as B is
                let q = Position {
                    x: 2 * a.x - b.x,
                    y: 2 * a.y - b.y,
                };

                // Check if position P is within the bounds of the map
                if p.x >= 0 && p.x < width && p.y >= 0 && p.y < height {
                    antinodes.insert(p); // Add P to the set of antinodes
                }

                // Check if position Q is within the bounds of the map
                if q.x >= 0 && q.x < width && q.y >= 0 && q.y < height {
                    antinodes.insert(q); // Add Q to the set of antinodes
                }
            }
        }
    }
    antinodes.len().to_string()
}

pub fn part2(input: &str) -> String {
    // Parse the input into a vector of string slices, each representing a row of the map
    let map: Vec<&str> = input.lines().collect();

    // Create a HashMap to group antennas by their frequency
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    // Iterate over each row and column to identify antennas and their positions
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell != '.' { // '.' represents an empty space; other characters are antennas
                antennas.entry(cell).or_insert_with(Vec::new)
                    .push(Position { x: x as isize, y: y as isize });
            }
        }
    }

    // Create a HashSet to store unique antinode positions
    let mut antinodes: HashSet<Position> = HashSet::new();

    // Iterate over each frequency and its corresponding list of antennas
    for (_freq, antenna_list) in &antennas {
        let n = antenna_list.len();
        if n < 2 {
            continue; // Skip frequencies with fewer than two antennas, as they cannot form lines
        }

        // Collect unique lines defined by all pairs of antennas for the current frequency
        let mut lines: HashSet<Line> = HashSet::new();

        for i in 0..n {
            for j in (i + 1)..n {
                let a = antenna_list[i];
                let b = antenna_list[j];
                let line = Line::from_positions(&a, &b); // Create a line from the pair
                lines.insert(line); // Add the line to the set of unique lines
            }
        }

        // Iterate over each unique line to identify all positions lying on that line
        for line in lines {
            // Iterate through each row and column of the map
            for (x, row) in map.iter().enumerate() {
                for (y, _cell) in row.chars().enumerate() {
                    let p = Position { x: x as isize, y: y as isize }; // Current position

                    // Check if the current position lies exactly on the line using the line equation
                    if line.a * p.x + line.b * p.y + line.c == 0 {
                        antinodes.insert(p); // Add the position to the set of antinodes
                    }
                }
            }
        }
    }
    antinodes.len().to_string()
}


#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

  #[test]
  fn part1_works() {
      let result = part1(INPUT);
      assert_eq!(result, "14");
  }

  #[test]
  fn part2_works() {
      let result = part2(INPUT);
      assert_eq!(result, "70");
  }
}
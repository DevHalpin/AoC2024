use starter::part1;
use starter::part2;
use get_input::fetch_input;


fn main() {
    let file = match fetch_input("6") {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to fetch input");
            return;
        }
    };
    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}
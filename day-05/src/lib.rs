use std::collections::{HashMap, HashSet, VecDeque};

// parse input into rules (a vector of tuples) and updates (a vector of i32 vectors)
fn parse_input(input: &str) -> (Vec<(i32,i32)>, Vec<Vec<i32>>) {
  let rules_and_updates: Vec<&str> = input.trim().split("\n\n").collect();

  let rules: Vec<(i32, i32)> = rules_and_updates[0].lines()
    .map(|line| {
      let pages: Vec<i32> = line.split('|').map(|page| page.parse().unwrap()).collect();
      (pages[0], pages[1])
    })
    .collect();
  println!("Parsed {} rules", rules.len());
  println!("{:?}", rules);

  let updates: Vec<Vec<i32>> = rules_and_updates[1].lines()
    .map(|line| {
      line.split(',').map(|page| page.parse().unwrap()).collect()
    })
    .collect();
  println!("Parsed {} updates", updates.len());
  println!("{:?}", updates);
  (rules, updates)
}

fn build_graph(rules: &Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
  let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
  // iterate over the rules and build the graph
  for (page1, page2) in rules {
    // get the value for page1 (a vector of its neighbors), or insert a new one
    let entry = graph.entry(*page1).or_insert(vec![]);
    // push the neighbor to the vector
    entry.push(*page2);
  }
  println!("Built graph with {} nodes", graph.len());
  println!("{:?}", graph);
  graph
}

fn get_valid_ordering(graph: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
  // hash map to hold nodes in the update
  let mut nodes: HashMap<i32, Vec<i32>> = HashMap::new();
  // hash map to hold the number of nodes that must be visited before a node
  let mut nodes_before: HashMap<i32, i32> = HashMap::new();
  // clone of updates
  let update_set: HashSet<i32> = update.iter().cloned().collect();

  // initialize nodes_before with 0 for each node in updates
  for &page in update {
    nodes_before.insert(page, 0);
  }

  // filter graph to include only nodes from the update and calculate number of nodes that must be before each node
  for (&page, neighbors) in graph {
    if update_set.contains(&page) {
      // filter neighbors to include only nodes from the update
      let valid_neighbours: Vec<i32> = neighbors.iter().filter(|&n| update_set.contains(&n)).cloned().collect();
      // insert the page and its valid neighbours into nodes
      nodes.insert(page, valid_neighbours.clone());
      // increment the number of nodes that must be visited before each node
      for &n in &valid_neighbours {
        let entry = nodes_before.entry(n).or_insert(0);
        *entry += 1;
      }
    }
  }

  //ready to perform topological sort at this point
  //initialize a queue
  let mut queue: VecDeque<i32> = nodes_before
    .iter()
    .filter_map(|(&page, &value)| if value == 0 {Some(page)} else {None})
    .collect();
  println!("Starting queue with nodes that do not require any other nodes to be visited first: {:?}", queue);

  //initialize a vector to hold the order of nodes
  let mut sorted_order = Vec::new();

  //perform topological sort
  while let Some(page) = queue.pop_front() {
    // push the node to the sorted order vector
    sorted_order.push(page);
    // if the node has neighbors
    if let Some(neighbors) = nodes.get(&page) {
      // iterate over the neighbors
      for &neighbor in neighbors {
        // if the neighbor is in the nodes_before map
        if let Some(before_node_count) = nodes_before.get_mut(&neighbor) {
          // decrement the number of nodes that must be visited before the neighbor
          *before_node_count -= 1;
          // if the neighbor does not require any other nodes to be visited first, add it to the queue
          if *before_node_count == 0 {
            queue.push_back(neighbor);
          }
        }
      }
    }
  }
  println!("Sorted order: {:?}", sorted_order);
  sorted_order
}


pub fn part1(input: &str) -> String {
  let (rules, updates) = parse_input(input);
  let graph = build_graph(&rules);
  let mut total = 0;

  for update in &updates {
    let valid_order = get_valid_ordering(&graph, update);
    let valid = valid_order == *update;
    if valid {
      let middle_page = update[update.len() / 2];
      println!("Middle page of update {:?}: {}", update, middle_page);
      total += middle_page;
    }
  }
  
  println!("Total: {}", total);
  total.to_string()
}

pub fn part2(input: &str) -> String {
  let (rules, updates) = parse_input(input);
  let graph = build_graph(&rules);
  let mut total = 0;
  

  for update in &updates {
    let valid_order = get_valid_ordering(&graph, update);
    let valid = valid_order == *update;
    if !valid {
      let middle_page = valid_order[valid_order.len() / 2];
      println!("Middle page of update {:?}: {}", update, middle_page);
      total += middle_page;
    }
  }
  
  println!("Total: {}", total);
  total.to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

  #[test]
  fn part1_works() {
      let result = part1(INPUT);
      assert_eq!(result, "143");
  }

  #[test]
  fn part2_works() {
      let result = part2(INPUT);
      assert_eq!(result, "123");
  }
}
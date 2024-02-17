use std::collections::HashMap;
use std::{fs, thread};
use std::string::ParseError;
use std::str::FromStr;

#[derive(Clone)]
struct Instructions {
  set: Vec<char>,
  currentInstruction: usize
}

impl Instructions {
  fn next(&mut self) -> char {

    let nextInstruction = self.set.get(self.currentInstruction).unwrap().clone();
    self.currentInstruction += 1;

    if self.currentInstruction >= self.set.len() {
      self.currentInstruction = 0;
    }

    nextInstruction
  }
}

impl FromStr for Instructions {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let set: Vec<char> = s.chars().collect();
    Ok(Instructions {
      set,
      currentInstruction: 0
    })
  } 
}

#[derive(Clone, Debug)]
struct Node {
  key: String,
  left: String,
  right: String
}

impl Node {
  fn goDirection(self: &Self, direction: char) -> String {
    match direction {
      'L' => self.left.to_string(),
      'R' => self.right.to_string(),
      _ => panic!("Unexpected direction")
    }
  }
}

impl FromStr for Node {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let key = s.chars().take(3).collect();
    let left = s.chars().skip(7).take(3).collect();
    let right = s.chars().skip(12).take(3).collect();

      Ok(Node {
        key,
        left,
        right
      })
  }
}

fn is_prime_number(number: i32) -> bool {
  for divisor in 2..number {
    if number % divisor == 0 {
      return false;
    }
  }

  true
}

fn get_primes_up_to(limit: i32) -> Vec<i32> {
  let mut prime_numbers = vec![];
  for i in 2..=limit {
    if is_prime_number(i) {
      prime_numbers.push(i);
    }
  }

  prime_numbers
}

pub fn part_1(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day8part1.txt");
  
  let contents = fs::read_to_string(inputFile).expect("File should exist");

  let mut lines = contents.lines();

  let mut instructions = Instructions::from_str(lines.next().unwrap()).unwrap();
  
  let _ = lines.next();
  
  let mut nodes: HashMap<String, Node> = HashMap::new();

  for line in lines {
    let node = Node::from_str(line).unwrap();
    nodes.insert(node.key.to_string(), node);
  }

  let starting_node_key: String = "AAA".to_string();
  let target_node_key: String = "ZZZ".to_string();

  let starting_node = nodes.get(&starting_node_key).unwrap();
  let target_node = nodes.get(&target_node_key).unwrap();
  let mut current_node = starting_node;

  let mut travel_count: u64 = 0;

  while current_node.key != target_node.key {

    let direction = instructions.next();
    let next_node_key = current_node.goDirection(direction);
    travel_count += 1;

    current_node = nodes.get(&next_node_key).unwrap();
  }

  println!("Day 8 Part 1: {}", travel_count);
}

pub fn part_2(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day8part1.txt");

  let contents = fs::read_to_string(inputFile).expect("File should exist");
    
  let mut lines = contents.lines();

  let instructions = Instructions::from_str(lines.next().unwrap()).unwrap();
  
  let _ = lines.next();
  
  let mut nodes: HashMap<String, Node> = HashMap::new();
  let mut start_nodes: Vec<Node> = vec![];

  for line in lines {
    let node = Node::from_str(line).unwrap();
    
    if node.key.ends_with('A') {
      start_nodes.push(node.clone());
    }

    nodes.insert(node.key.to_string(), node);

  }

  let mut handles = vec![];

  for node in start_nodes {
    let handle = thread::spawn({ 
      
      let starting_node = node.clone();
      let nodes = nodes.clone();

      // Target node is any that end in Z      
      let mut current_node = starting_node;
      let mut instructions = instructions.clone();
      move || {

        let mut travel_count = 0;

        while !current_node.key.ends_with('Z') {
          let direction = instructions.next();
          let next_node_key = current_node.goDirection(direction);
          travel_count += 1;

          current_node = nodes.get(&next_node_key).unwrap().clone();
        }

        travel_count
      }
    }); 

    handles.push(handle);
  }

  let mut travelCounts: Vec<i32> = vec![];

  for handle in handles {
    let handleValue = handle.join().unwrap();
    travelCounts.push(handleValue);
  }
  
  // Solve for LCM

  let prime_numbers: Vec<i32> = get_primes_up_to(500);
  let mut max_prime_factors = HashMap::new();

  for distance in travelCounts {
    
    let mut prime_factors = HashMap::new();

    let mut remainingDistance = distance;

    for number in prime_numbers.iter() {

      if remainingDistance == 1 {
        break;
      }
      
      while remainingDistance % number == 0 {
        *prime_factors.entry(number).or_insert(0) += 1;
        remainingDistance = remainingDistance / number;
      }
    }

    for (factor, power) in prime_factors {
      let max_power: i32 = *max_prime_factors.get(factor).unwrap_or(&0);

      if max_power < power {
        *max_prime_factors.entry(factor).or_insert(0) = power;
      }
    }
  }

  let mut least_common_multiple = 1;

  for (factor, power) in max_prime_factors.into_iter() {
    least_common_multiple *= u64::pow(*factor as u64, power as u32);
  }

  println!("Day 8 Part 2: {}", least_common_multiple);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_1_using_test_file() {
    let inputFile = "./inputs/day8test.txt";

    part_1(Some(inputFile));

    // should print 6
  }

  #[test]
  fn part_2_using_test_file() {
    let inputFile = "./inputs/day8test.txt";

    part_2(Some(inputFile));

    // should print 5905
  }
}
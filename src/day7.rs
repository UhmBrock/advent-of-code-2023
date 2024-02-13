use std::fs;

pub fn part_1(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day6part1.txt");
  
  let contents = fs::read_to_string(inputFile).expect("File should exist");

  println!("Day 7 Part 1: ");
}

pub fn part_2(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day6part1.txt");

  let contents = fs::read_to_string(inputFile).expect("File should exist");
  
  println!("Day 6 Part 2: ");
}
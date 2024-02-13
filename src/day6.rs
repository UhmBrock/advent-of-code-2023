use std::fs;

fn get_winning_results(race: (&u64, &u64)) -> Vec<u64> {
  let raceTimeMs = race.0;
  let raceRecord = race.1;

  let mut winningResults = vec![];

  for ms_held in 1..=*raceTimeMs {
    let distance = ms_held * (raceTimeMs - ms_held);

    if distance > *raceRecord {
      winningResults.push(ms_held);
    }
  }

  return winningResults;
}

pub fn part_1(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day6part1.txt");
  
  let contents = fs::read_to_string(inputFile).expect("File should exist");
  let mut contents = contents.lines();

  let times: Vec<u64> = contents.next().unwrap().split_whitespace().skip(1).map(|t| t.parse().unwrap()).collect();
  let distances: Vec<u64> = contents.next().unwrap().split_whitespace().skip(1).map(|d| d.parse().unwrap()).collect();
  
  let races: Vec<(&u64, &u64)> = times.iter().zip(distances.iter()).collect();

  let mut resultProduct = 1;

  for race in races {
    resultProduct *= get_winning_results(race).len();
  }

  println!("Day 6 Part 1: {}", resultProduct);
}

pub fn part_2(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day6part1.txt");

  let contents = fs::read_to_string(inputFile).expect("File should exist");
  let mut contents = contents.lines();

  let times: Vec<&str> = contents.next().unwrap().split_whitespace().skip(1).collect();
  let times = times.concat().parse::<u64>().unwrap();
  let distances: Vec<&str> = contents.next().unwrap().split_whitespace().skip(1).collect();
  let distances = distances.concat().parse::<u64>().unwrap();
  
  let race = (&times, &distances);

  let result = get_winning_results(race).len();
  
  println!("Day 6 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_1_using_test_file() {
    let inputFile = "./inputs/day6test.txt";

    part_1(Some(inputFile));

    // should print 288
  }

  #[test]
  fn part_2_using_test_file() {
    let inputFile = "./inputs/day6test.txt";

    part_2(Some(inputFile));

    // should print 71503
  }
}
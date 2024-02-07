use crate::utils::file_utils;
use std::str::FromStr;

pub fn part_1(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day2part1.txt");

  let maxGreen = 13;
  let maxRed = 12;
  let maxBlue = 14;

  let mut possibleGameIdSum = 0;

  if let Ok(lines) = file_utils::read_lines(inputFile) {
    for line in lines.flatten() {
      let round = GameRound::from_str(line.as_str()).unwrap();

      if round.is_possible(maxGreen, maxRed, maxBlue) {
        possibleGameIdSum += round.id;
      }
    }
  }

  println!("Day 2 Part 1: {}", possibleGameIdSum);

}

pub fn part_2(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day2part1.txt");

  let mut powerSum = 0;

  if let Ok(lines) = file_utils::read_lines(inputFile) {
    for line in lines.flatten() {
      let round = GameRound::from_str(line.as_str()).unwrap();

      let minSet = round.get_max_shown();

      powerSum += minSet.blue * minSet.red * minSet.green;
    }
  }

  println!("Day 2 Part 2: {}", powerSum);
}

#[derive(Debug)]
struct GameRound {
  id: i32,
  sets: Vec<GameSet>
}

impl GameRound {
  fn new() -> Self {
    GameRound { id: 0, sets: vec![] }
  }

  fn is_possible(self: &Self, maxGreen: i32, maxRed: i32, maxBlue: i32) -> bool {
    let maxSet = self.get_max_shown();

    return 
      maxSet.green <= maxGreen && 
      maxSet.red   <= maxRed && 
      maxSet.blue  <= maxBlue;
  }

  fn get_max_shown(self: &Self) -> GameSet {

    let mut maxSet = GameSet::new();

    for set in &self.sets {

      if set.blue > maxSet.blue {
        maxSet.blue = set.blue;
      }

      if set.green > maxSet.green {
        maxSet.green = set.green;
      }

      if set.red > maxSet.red {
        maxSet.red = set.red;
      }

    }

    return maxSet;
  }
}

impl Default for GameRound {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for GameRound {

  type Err = std::num::ParseIntError;

  fn from_str(gameLine: &str) -> Result<Self, Self::Err> {

    let mut parts = gameLine.split(":");

    let idString = parts.next().expect("Should always be a game id string");
    let idString = idString.split(" ").last().expect("Should always be an id in the Game Id String");
    let id: i32 = idString.parse::<i32>().unwrap();

    let resultSets = parts.next().expect("Should always be a result set string");
    let resultSetStrings = resultSets.split(";");
    let sets: Vec<GameSet> = resultSetStrings.map(|setString| GameSet::from_str(setString).unwrap()).collect();

    Ok(GameRound { id, sets } )
  }


}

#[derive(Debug, Clone, Copy)]
struct GameSet {
  green: i32,
  red: i32,
  blue: i32
}

impl GameSet {

  fn new() -> Self {
    GameSet { green: 0, red: 0, blue: 0 }
  }
}

impl Default for GameSet {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for GameSet {
  type Err = std::num::ParseIntError;

  fn from_str(setString: &str) -> Result<Self, Self::Err> {

    let mut gameSet = GameSet::new();

    let sets = setString.split(";");
    for set in sets {
      let colors = set.split(',');

      for color in colors {
        let mut colorParts = color.trim().split(' ');
        let count = colorParts.next().expect("Should always be a color count");
        let count = count.parse::<i32>().unwrap();
        let color = colorParts.next().expect("Should always be a color");

        match color {
          "red" => gameSet.red = count,
          "blue" => gameSet.blue = count,
          "green" => gameSet.green = count,
          &_ => panic!("Unexpected color name")
        }
      }      
    }
    
    Ok(gameSet)
  }
}
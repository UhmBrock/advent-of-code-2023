use crate::utils::file_utils;

pub fn part_1(inputFile: Option<&str>) {

  let inputFile = inputFile.unwrap_or("./inputs/day1part1.txt");

  if let Ok(lines) = file_utils::read_lines(inputFile) {

    let mut calibration_sum: u32 = 0;

    for line in lines.flatten() {

      let calibration_value = extract_calibration_value(&line);
      
      calibration_sum = calibration_sum + calibration_value;
    
    }

    println!("Day 1 Part 1: {}", calibration_sum);
  }
}

pub fn part_2(inputFile: Option<&str>) {

  let inputFile = inputFile.unwrap_or("./inputs/day1part1.txt");

  if let Ok(lines) = file_utils::read_lines(inputFile) {

    let mut calibration_sum: u32 = 0;

    for mut line in lines.flatten() {

      let line = parse_numerics_from_string(&mut line);

      let calibration_value = extract_calibration_value(&line);
      
      calibration_sum = calibration_sum + calibration_value;
    
    }

    println!("Day 1 Part 2: {}", calibration_sum);
  }

}

fn parse_numerics_from_string(line: &mut String) -> String {

  let numeric_words = [
    ("zero", '0'),
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9')
  ];

  let mut number_locations: Vec<(usize, char)> = vec![];

  for word in numeric_words {

    let wordIndices = line.match_indices(word.0);

    for wordIndex in wordIndices {
      number_locations.push((wordIndex.0, word.1));
    }

    for char in line.char_indices() {
      if char.1 == word.1 {
        number_locations.push(char);
      }
    }
  }  

  number_locations.sort_by_key(|k| k.0);

  let lineNumbers = number_locations.into_iter()
    .map(|value| value.1.to_string())
    .collect::<String>();

  return lineNumbers;
}

fn extract_calibration_value(line: &String) -> u32 {

  let mut firstDigit: Option<char> = None;
  let mut lastDigit: Option<char> = None;

  for char in line.chars() {
    if let Some(_) = char.to_digit(10) {

      if firstDigit.is_none() {
        firstDigit = Some(char.clone());
      }
      lastDigit = Some(char.clone());
    }
  }

  let mut calibration_value = String::from(firstDigit.unwrap());
  calibration_value.push(lastDigit.unwrap());

  calibration_value.parse().unwrap()
}

#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn test_extract_calibration_value() {
    assert_eq!(extract_calibration_value(&String::from("abc123def")), 13);
    assert_eq!(extract_calibration_value(&String::from("abc1234def")), 14);
    assert_eq!(extract_calibration_value(&String::from("abc12345def")), 15);
  }

  #[test]
  fn test_convert_numeric_words_ordered_first_to_last()  {
    assert_eq!(parse_numerics_from_string(&mut String::from("zoneight234")), "18234");
    assert_eq!(parse_numerics_from_string(&mut String::from("zeightwo234")), "82234");
  }  

  #[test]
  fn test_convert_numeric_words() {
    let mut line = String::from("one two three four five six seven eight nine zero");
    assert_eq!(parse_numerics_from_string(&mut line), "1234567890");
    assert_eq!(parse_numerics_from_string(&mut String::from("abc123defour")), "1234");
    assert_eq!(parse_numerics_from_string(&mut String::from("abc1234dthreefourthree")), "1234343");
    assert_eq!(parse_numerics_from_string(&mut String::from("abtwo12345defour")), "2123454");
  }
}
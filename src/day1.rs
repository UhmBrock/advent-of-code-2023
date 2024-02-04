use crate::utils::file_utils;

pub fn part_1() {

  let inputFile = "./inputs/day1part1.txt";

  if let Ok(lines) = file_utils::read_lines(inputFile) {

    let mut calibration_sum: u32 = 0;

    for line in lines.flatten() {
      let calibration_value = extract_calibration_value(line);
      
      calibration_sum = calibration_sum + calibration_value;
      
      println!("+{}, {}", calibration_value, calibration_sum);
    }

    println!("{}", calibration_sum);
  }
}

fn extract_calibration_value(line: String) -> u32 {

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
use std::{
  borrow::Borrow, collections::HashSet, fs, str::FromStr, string::ParseError
};

pub fn part_1(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day3part1.txt");

  let contents = fs::read_to_string(inputFile).expect("File should exist");

  let schematic = EngineSchematic::from_str(contents.as_str()).unwrap();

  let possible_starts = schematic.find_start_of_all_possible_parts();
  
  let mut part_sum: i32 = 0;

  for start in possible_starts {
    if let Some(part) = schematic.get_part_at_location(start) {
      part_sum += part.parse::<i32>().unwrap();
    }
  }

  println!("Day 3 Part 1: {}", part_sum);
}

pub fn part_2(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day3part1.txt");
  let contents = fs::read_to_string(inputFile).expect("File should exist");
  let _schematic = EngineSchematic::from_str(contents.as_str()).unwrap();

}

#[derive(Default, Debug)]
struct EngineSchematic {
  components: Vec<Vec<EngineComponent>>
}

impl EngineSchematic {

  fn get_position(self: &Self, position: &Position) -> Option<&EngineComponent> {

    let column = self.get_x(position.x as usize)?;
    let value = column.get(position.y as usize);

    return value;
  }

  fn get_x(self: &Self, x: usize) -> Option<&Vec<EngineComponent>> {
    self.components.get(x)
  }

  fn get_x_mut(self: &mut Self, x: usize) -> Option<&mut Vec<EngineComponent>> {
    self.components.get_mut(x)
  }
  
  fn find_start_of_possible_part(self: &Self, component: &EngineComponent) -> Option<Position> {
    if !component.is_possible_part() {
      return None;
    }

    if let Some(previousPosition) = component.position.previous() {

      let previousComponent = self.get_position(&previousPosition).unwrap();
      
      if previousComponent.is_possible_part() {
        // The part continues on
        return self.find_start_of_possible_part(previousComponent);
      } else {
        // We found the start of the part
        return Some(component.position);
      }
    } else {
      // Has to be the start if it's the start of the row
      return Some(component.position);
    }
  }

  fn find_start_of_all_possible_parts(self: &Self) -> HashSet<Position> {

    let mut possible_parts_starts: HashSet<Position> = HashSet::new();

    for col in &self.components {
      for component in col {        
        let part_start = self.find_start_of_possible_part(component);
        if part_start.is_some() {
          possible_parts_starts.insert(part_start.unwrap());
        }
      }
    }

    return possible_parts_starts;
  }

  fn get_part_at_location(self: &Self, position: Position) -> Option<String> {
    
    let mut partString = String::new();
    let mut is_confirmed_part = false;
    
    let mut currentPosition = position;

    loop {
      let currentComponent = self.get_position(&currentPosition);

      if currentComponent.is_none() {
        break;
      }

      let currentComponent = currentComponent.unwrap();

      if !currentComponent.is_possible_part() {
        break;
      }

      partString.push(currentComponent.value);
      
      if self.position_touches_symbol(currentComponent) {
        is_confirmed_part = true;
      }
      
      currentPosition = currentPosition.next().unwrap();
      // println!("Checking pos: {:?}, current string: {}, is part: {}", currentPosition, partString, is_confirmed_part);
    }

    if !is_confirmed_part {
      return None;
    }

    return Some(partString);
  }

  fn position_touches_symbol(self: &Self, component: &EngineComponent) -> bool {

    let adjacentPositions = [
      component.position.fromOffset(-1, -1), // top left
      component.position.fromOffset(0, -1), // top
      component.position.fromOffset(1, -1), // top right
      component.position.fromOffset(-1, 0), // left
      component.position.fromOffset(1, 0), // right
      component.position.fromOffset(-1, 1), // bottom left
      component.position.fromOffset(0, 1),  // bottom
      component.position.fromOffset(1, 1), // bottom right
    ];

    for pos in adjacentPositions {
      let component = self.get_position(&pos);

      if component.is_none() {
        continue;
      }

      let component = component.unwrap();

      if component.is_symbol() {
        return true;
      }
    }
 
    return false;
  }

  #[allow(dead_code)]
  fn get_all_gears(self: &Self) -> Vec<&EngineComponent> {
    todo!()
  }

  #[allow(dead_code)]
  fn component_is_gear(self: &Self, _component: &EngineComponent) -> bool {

    todo!()

  }
}

impl FromStr for EngineSchematic {
  type Err = ParseError;

  fn from_str(inputString: &str) -> Result<Self, Self::Err> {
    let mut schematic = Self::default();

    for line in inputString.lines() {

      for x in 0..line.len() {
        if schematic.components.get(x).is_none() {
          schematic.components.push(vec![])
        }
      }

      for (xPos, char) in line.char_indices() {
        
        let yPos = schematic.get_x(xPos).expect("Should be initialized").len();

        let component = EngineComponent {
          position: Position { x: xPos as isize, y: yPos as isize},
          value: char
        };

        schematic.get_x_mut(xPos).expect("Should be initialized").push(component);
      }
    }

    Ok(schematic)
  }
}

#[derive(Default, Debug)]
struct EngineComponent {
  position: Position,
  value: char
}

impl EngineComponent {
  fn is_symbol(self: &Self) -> bool {
    match self.value.borrow() {
      '*' => true,
      '#' => true,
      '-' => true,
      '+' => true,
      '@' => true,
      '%' => true,
      '&' => true,
      '=' => true,
      '$' => true,
      '/' => true,
      &_ => false
    }
  }

  // fn is_gear(self: &Self) -> bool {
  //   match self.value.borrow() {
  //     '*' => true,
  //     &_ => false
  //   }
  // }

  fn is_possible_part(self: &Self) -> bool {
    self.value.is_numeric()
  }
}


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  x: isize,
  y: isize
}

impl Position {
  fn fromOffset(self: &Self, xOffset: isize, yOffset: isize) -> Position {
    Position {
      x: self.x + xOffset,
      y: self.y + yOffset
    }
  }

  fn previous(self: &Self) -> Option<Position> {
    if self.x <= 0 {
      return None;
    }

    Some(Position { x: self.x - 1, y: self.y })
  }

  fn next(self: &Self) -> Option<Position> {
    Some(Position { x: self.x + 1, y: self.y })
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn schematic_builds_correctly() {
    let inputString = 
    "01234\n\
    12345\n\
    23456\n\
    34567\n\
    45678";

    let schematic = EngineSchematic::from_str(inputString).unwrap();

    assert_eq!(schematic.get_position(&Position { x: 0, y: 0 }).unwrap().value, '0');
    assert_eq!(schematic.get_position(&Position { x: 0, y: 4 }).unwrap().value, '4');
    assert_eq!(schematic.get_position(&Position { x: 4, y: 4 }).unwrap().value, '8');
    assert_eq!(schematic.get_position(&Position { x: 4, y: 0 }).unwrap().value, '4');
    assert_eq!(schematic.get_position(&Position { x: 2, y: 2 }).unwrap().value, '4');
    assert_eq!(schematic.components.len(), 5);
    assert_eq!(schematic.get_x(1).expect("Should be initialized").len(), 5);
  }

  #[test]
  fn possible_part_starts_calculate_correctly() {
    let inputString = 
    ".....\n\
    .11..\n\
    ...22\n\
    33333\n\
    .44.5";

    let schematic = EngineSchematic::from_str(inputString).unwrap();

    let possiblePartStarts = schematic.find_start_of_all_possible_parts();

    assert!(possiblePartStarts.contains(&Position { x: 1, y: 1 }));
    assert!(possiblePartStarts.contains(&Position { x: 3, y: 2 }));
    assert!(possiblePartStarts.contains(&Position { x: 0, y: 3 }));
    assert!(possiblePartStarts.contains(&Position { x: 1, y: 4 }));
    assert!(possiblePartStarts.contains(&Position { x: 4, y: 4 }));
  }

  #[test]
  fn possible_part_ends_calculate_correctly() {
    let inputString = 
    ".....\n\
    .11..\n\
    @..22\n\
    33333\n\
    .44.5";

    let schematic = EngineSchematic::from_str(inputString).unwrap();

    assert_eq!(schematic.get_part_at_location(Position { x: 0, y: 0 }), None);
    assert_eq!(schematic.get_part_at_location(Position { x: 1, y: 1 }), Some("11".to_string()));
    assert_eq!(schematic.get_part_at_location(Position { x: 3, y: 2 }), None); // 22 does not touch symbol
    assert_eq!(schematic.get_part_at_location(Position { x: 0, y: 3 }), Some("33333".to_string()));
  }
}
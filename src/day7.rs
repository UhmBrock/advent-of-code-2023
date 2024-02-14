use std::{cmp, collections::HashMap, fs, str::FromStr, string::ParseError};
#[derive(Debug)]
struct Card {
  label: String,
  strength: i32
}

impl FromStr for Card {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      
      let card: Card = match s {
        "A" => Card { label: s.to_string(), strength: 13 },
        "K" => Card { label: s.to_string(), strength: 12 },
        "Q" => Card { label: s.to_string(), strength: 11 },
        "T" => Card { label: s.to_string(), strength: 10 },
        "9" => Card { label: s.to_string(), strength: 9 },
        "8" => Card { label: s.to_string(), strength: 8 },
        "7" => Card { label: s.to_string(), strength: 7 },
        "6" => Card { label: s.to_string(), strength: 6 },
        "5" => Card { label: s.to_string(), strength: 5 },
        "4" => Card { label: s.to_string(), strength: 4 },
        "3" => Card { label: s.to_string(), strength: 3 },
        "2" => Card { label: s.to_string(), strength: 2 },
        "J" => Card { label: s.to_string(), strength: 1 },
        _ => panic!("Unhandled card")
      };

      Ok(card)
  }
}

#[derive(Debug)]
#[allow(dead_code)]
struct HandType {
  label: String,
  strength: i32
}

#[derive(Debug)]
struct Hand<const WILDCARD_ACTIVE: bool> {
  cards: Vec<Card>,
  bid: i32,
  handType: HandType
}

impl<const WILDCARD_ACTIVE: bool> Eq for Hand<WILDCARD_ACTIVE> {}

impl<const WILDCARD_ACTIVE: bool> PartialEq for Hand<WILDCARD_ACTIVE> {
  fn eq(&self, other: &Self) -> bool {
    self.handType.strength == other.handType.strength && self.cards.iter().map(|c| &c.strength).eq(other.cards.iter().map(|c| &c.strength))
  }
}

impl<const WILDCARD_ACTIVE: bool> PartialOrd for Hand<WILDCARD_ACTIVE> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {

    if self.handType.strength == other.handType.strength {
      for i in 0..self.cards.len() {
        if self.cards[i].strength == other.cards[i].strength {
          continue;
        }

        return self.cards[i].strength.partial_cmp(&other.cards[i].strength); 
      }
    }

    self.handType.strength.partial_cmp(&other.handType.strength)
  }
}

impl<const WILDCARD_ACTIVE: bool> Ord for Hand<WILDCARD_ACTIVE> {
  fn cmp(&self, other: &Self) -> cmp::Ordering {

    if self.handType.strength == other.handType.strength {
      for i in 0..self.cards.len() {
        if self.cards[i].strength == other.cards[i].strength {
          continue;
        }

        return self.cards[i].strength.cmp(&other.cards[i].strength); 
      }
    }

    self.handType.strength.cmp(&other.handType.strength)
  }
}

impl Hand<false> {
  fn get_hand_type(cards: &Vec<Card>) -> HandType {
    
    let mut cardHash = HashMap::new();

    for card in cards.iter() {
      *cardHash.entry(&card.label).or_insert(0) += 1;
    }

    if *cardHash.values().max().unwrap() == 5 {
      return HandType {
        label: "Five of a kind".into(),
        strength: 7
      };
    }
    
    if *cardHash.values().max().unwrap() == 4 {
      return HandType {
        label: "Four of a kind".into(),
        strength: 6
      };
    }

    if *cardHash.values().max().unwrap() == 3 && cardHash.values().find(|v| **v == 2).is_some() {
      return HandType {
        label: "Full house".into(),
        strength: 5
      };
    }

    if *cardHash.values().max().unwrap() == 3 {
      return HandType {
        label: "Three of a kind".into(),
        strength: 4
      };
    }

    if *cardHash.values().max().unwrap() >= 2 {
      let mut pairs_found = 0;
      for card in cardHash.iter() {
        if *card.1 >= 2 {
          pairs_found += 1;
        }
      }

      if pairs_found == 2 {
        return HandType {
          label: "Two pair".into(),
          strength: 3
        };
      }
    }

    if cardHash.values().max() == Some(&2) {
      return HandType {
        label: "One pair".into(),
        strength: 2
      };
    }

    HandType {
      label: "High card".into(),
      strength: 1
    }
  }
}

impl Hand<true> {
  fn get_hand_type(cards: &Vec<Card>) -> HandType {
    
    let mut cardHash = HashMap::new();

    let mut jokerCount = 0;

    for card in cards.iter() {
      
      if card.label == "J" {
        jokerCount += 1;
        continue;
      } 

      *cardHash.entry(&card.label).or_insert(0) += 1;
    }

    if jokerCount >= 5 || *cardHash.values().max().unwrap() >= (5 - jokerCount) {
      return HandType {
        label: "Five of a kind".into(),
        strength: 7
      };
    }
    
    if *cardHash.values().max().unwrap() >= (4 - jokerCount) {
      return HandType {
        label: "Four of a kind".into(),
        strength: 6
      };
    }

    // Jokers turn a two pair into a full house, any other combo becomes a four of a kind
    if jokerCount == 1 && *cardHash.values().max().unwrap() == 2 {
      let mut pairs_found = 0;
      for card in cardHash.iter() {
        if *card.1 >= 2 {
          pairs_found += 1;
        }
      }

      if pairs_found == 2 {
        return HandType {
          label: "Full house".into(),
          strength: 5
        };
      }
    } else if *cardHash.values().max().unwrap() == 3 && cardHash.values().find(|v| **v == 2).is_some() {
      return HandType {
        label: "Full house".into(),
        strength: 5
      };
    }

    if *cardHash.values().max().unwrap() >= (3 - jokerCount) {
      return HandType {
        label: "Three of a kind".into(),
        strength: 4
      };
    }

    // Jokers don't affect two pair, it would just become a three of a kind
    if *cardHash.values().max().unwrap() == 2 {
      let mut pairs_found = 0;
      for card in cardHash.iter() {
        if *card.1 >= 2 {
          pairs_found += 1;
        }
      }

      if pairs_found == 2 {
        return HandType {
          label: "Two pair".into(),
          strength: 3
        };
      }
    }

    if *cardHash.values().max().unwrap() >= (2 - jokerCount) {
      return HandType {
        label: "One pair".into(),
        strength: 2
      };
    }

    HandType {
      label: "High card".into(),
      strength: 1
    }
  }
}

impl<const WILDCARD_ACTIVE: bool> FromStr for Hand<WILDCARD_ACTIVE> {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split_whitespace();

    let hand = parts.next().unwrap();
    let bid: i32 = parts.next().unwrap().parse().unwrap();

    let cards: Vec<Card> = hand.chars().map(|c| Card::from_str(&c.to_string()).unwrap()).collect();

    let handType = if WILDCARD_ACTIVE {
      Hand::<true>::get_hand_type(&cards)
    } else {
      Hand::<false>::get_hand_type(&cards)
    };

    Ok(Hand {
      cards,
      bid,
      handType
    })
  }
}

pub fn part_1(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day7part1.txt");
  
  let contents = fs::read_to_string(inputFile).expect("File should exist");

  let mut hands: Vec<Hand<false>> = vec![];

  for line in contents.lines() {
    let hand = Hand::from_str(line).unwrap();
    hands.push(hand);
  }

  hands.sort();

  let mut totalWinnings = 0;

  for (index, hand) in hands.iter().enumerate() {
    totalWinnings += (index+1) as i32 * hand.bid;    
  }
  println!("Day 7 Part 1: {}", totalWinnings);
}

pub fn part_2(inputFile: Option<&str>) {
  let inputFile = inputFile.unwrap_or("./inputs/day7part1.txt");

  let contents = fs::read_to_string(inputFile).expect("File should exist");
  
  let mut hands: Vec<Hand<true>> = vec![];

  for line in contents.lines() {
    let hand = Hand::from_str(line).unwrap();
    hands.push(hand);
  }

  hands.sort();

  let mut totalWinnings = 0;

  for (index, hand) in hands.iter().enumerate() {
    totalWinnings += (index+1) as i32 * hand.bid;    
  }
  println!("Day 7 Part 2: {}", totalWinnings);
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_1_using_test_file() {
    let inputFile = "./inputs/day7test.txt";

    part_1(Some(inputFile));

    // should print 6440
  }

  #[test]
  fn part_2_using_test_file() {
    let inputFile = "./inputs/day7test.txt";

    part_2(Some(inputFile));

    // should print 5905
  }
}
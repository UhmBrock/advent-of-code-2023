use std::collections::HashMap;
use std::{fs, result};
use std::str::FromStr;
use std::string::ParseError;

#[derive(Default, Debug)]
struct Card {
  id: i32,
  winningNumbers: Vec<i32>,
  revealedNumbers: Vec<i32>,
  matchingNumbers: Vec<i32>,
  resultingCards: Vec<i32>,
  totalResultingCards: i32,
  solved: bool
}

impl Card {
  fn getPoints(self: &Self) -> i32 {
    let match_count: u32 = self.matchingNumbers.len() as u32;
    
    if match_count <= 1 {
      return match_count as i32;
    }

    let points: i32 = 1 * 2_i32.pow(match_count - 1);
    return points;
  }
}

impl FromStr for Card {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    
    let mut new_card = Card::default();

    let mut card_parts = s.split(":");

    let card_id = card_parts.next().unwrap().split(" ").last().unwrap();
    new_card.id = card_id.parse::<i32>().unwrap();

    let mut card_parts = card_parts.next().unwrap().split("|");

    let winning_numbers = card_parts.next().unwrap().trim().split_whitespace();
    let revealed_numbers = card_parts.next().unwrap().trim().split_whitespace();

    for number in winning_numbers {
      let number = number.parse::<i32>().unwrap();
      new_card.winningNumbers.push(number);
    }

    for number in revealed_numbers {
      let number = number.parse::<i32>().unwrap();
      new_card.revealedNumbers.push(number);

      if new_card.winningNumbers.contains(&number) {
        new_card.matchingNumbers.push(number);
      }
    }

    if new_card.matchingNumbers.len() > 0 {

      let endingId = new_card.id + new_card.matchingNumbers.len() as i32;
      let idRange = new_card.id+1..=endingId;

      new_card.resultingCards = idRange.collect();
    }

    return Ok(new_card);
  }
}

fn get_cards(contents: String) -> HashMap<i32, Card> {

  let mut cards_hashset: HashMap<i32, Card> = HashMap::new();

  for line in contents.lines() {
    let card = Card::from_str(line).unwrap();

    cards_hashset.insert(card.id, card);
  }

  return cards_hashset;
}

fn get_points_sum(cards: HashMap<i32, Card>) -> i32 {

  let mut sum = 0;

  for (id, card) in cards {
    sum += card.getPoints();
  }

  return sum;
}

pub fn part_1() {
  let inputFile = "./inputs/day4part1.txt";
  let contents = fs::read_to_string(inputFile).expect("File should exist");

  let cards = get_cards(contents);
  let pointSum = get_points_sum(cards);

  println!("Day 4 Part 1: {}", pointSum);
}

pub fn part_2() {
  let inputFile = "./inputs/day4part1.txt";
  let contents = fs::read_to_string(inputFile).expect("File should exist");

  let mut cards = get_cards(contents);

  let mut total_card_count = cards.len() as i32;

  let mut cards_won = vec![];
  for (id, card) in cards.iter() {
    for resultCard in card.resultingCards.iter() {
      cards_won.push(resultCard);
    }
  }

  total_card_count += cards_won.len() as i32;

  while cards_won.len() > 0 {

    let cards_won_clone = cards_won.clone();
    cards_won.clear();

    for id in cards_won_clone {
      let card = cards.get(&id).unwrap();

      for resultCard in card.resultingCards.iter() {
        cards_won.push(resultCard);
      }
    }

    total_card_count += cards_won.len() as i32;
  }

  println!("Day 4 Part 2: {}", total_card_count);
}
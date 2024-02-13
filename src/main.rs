#![allow(non_snake_case)]

use std::thread::{self, JoinHandle};

pub mod utils;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

fn main() {

  let mut handles = vec![];

  day1::part_1(None);    
  day1::part_2(None);
  day2::part_1(None);
  day2::part_2(None);
  day3::part_1(None);
  day4::part_1(); 
  handles.push(as_thread(day4::part_2));
  day5::part_1();
  handles.push(as_thread(day5::part_2));
  day6::part_1(None);
  day6::part_2(None);

  println!("=====================================");
  
  day7::part_1(None);

  if false {
      for handle in handles {
          handle.join().unwrap();
      }
  }
}

fn as_thread(function: fn()) -> JoinHandle<()> {
  thread::spawn(move || {
      function()
  })
}
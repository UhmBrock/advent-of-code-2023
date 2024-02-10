use std::string::ParseError;
use std::{fs, ops::Range};
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
struct Map {
  destination_range: Range<u64>,
  source_range: Range<u64>,
  length: u64
}
impl FromStr for Map {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {

      let mut parts = s.split_whitespace();

      let destinationStart = parts.next().unwrap().trim().parse::<u64>().unwrap();
      let sourceStart = parts.next().unwrap().trim().parse::<u64>().unwrap();
      let length = parts.next().unwrap().trim().parse::<u64>().unwrap();
     
      let destinationEnd = destinationStart + length + 1;
      let sourceEnd = sourceStart + length + 1;

      let new_map = Map {
        destination_range: destinationStart..destinationEnd,
        source_range: sourceStart..sourceEnd,
        length
      };

      return Ok(new_map);
  }
}

struct Mapper {
  prevMaps: Vec<Map>, 
  nextMaps: Vec<Map>
}

impl Mapper {

  fn new(prevMaps: Vec<Map>, nextMaps: Vec<Map>) -> Self {
    Mapper {
      prevMaps: prevMaps,
      nextMaps: nextMaps
    }
  }
  
  fn prev(self: &Self, value: u64) -> u64 {
    // TODO better choose which map to use
    for map in self.prevMaps.iter() {
      if map.destination_range.contains(&value) {
        let destinationOffset = value - map.destination_range.start;
        return map.source_range.start + destinationOffset;
      }
    }

    return value;
  }

  fn next(self: &Self, value: u64) -> u64 {
    // TODO better choose which map to use
    for map in self.nextMaps.iter() {
      if map.source_range.contains(&value) {
        let sourceOffset = value - map.source_range.start;
        return map.destination_range.start + sourceOffset;
      }
    }

    return value;
  }
}

fn translate_using_map(value: &u64, map: &Map) -> Option<u64> {
  if map.source_range.contains(value) {
    let sourceOffset = value - map.source_range.start;
    return Some(map.destination_range.start + sourceOffset);
  }

  return None;
}

pub fn part_1() {
    let inputFile = "./inputs/day5part1.txt";
    let contents = fs::read_to_string(inputFile).expect("File should exist");

    let mut parts = contents.split("\r\n\r\n");

    let seed_string = parts.next().unwrap().split(':').last().unwrap().trim();
    let seed_to_soil = parts.next().unwrap().split(':').last().unwrap().trim();
    let soil_to_fertilizer = parts.next().unwrap().split(':').last().unwrap().trim();
    let fertilizer_to_water = parts.next().unwrap().split(':').last().unwrap().trim();
    let water_to_light = parts.next().unwrap().split(':').last().unwrap().trim();
    let light_to_temperature = parts.next().unwrap().split(':').last().unwrap().trim();
    let temperature_to_humidity = parts.next().unwrap().split(':').last().unwrap().trim();
    let humidity_to_location = parts.next().unwrap().split(':').last().unwrap().trim();

    let seeds: Vec<u64> = seed_string.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

    let seed_to_soil_maps: Vec<Map> = seed_to_soil.lines().map(|str| Map::from_str(str).unwrap() ).collect();
    let soil_to_fertilizer_maps: Vec<Map> = soil_to_fertilizer.lines().map(|str| Map::from_str(str).unwrap() ).collect();
    let fertilizer_to_water_maps: Vec<Map> = fertilizer_to_water.lines().map(|str| Map::from_str(str).unwrap() ).collect();
    let water_to_light_maps: Vec<Map> = water_to_light.lines().map(|str| Map::from_str(str).unwrap() ).collect();
    let light_to_temperature_maps: Vec<Map> = light_to_temperature.lines().map(|str| Map::from_str(str).unwrap() ).collect();
    let temperature_to_humidity_maps: Vec<Map> = temperature_to_humidity.lines().map(|str| Map::from_str(str).unwrap() ).collect();
    let humidity_to_location_maps: Vec<Map> = humidity_to_location.lines().map(|str| Map::from_str(str).unwrap() ).collect();

    let seed_mapper = Mapper::new(vec![], seed_to_soil_maps.clone());
    let soil_mapper = Mapper::new(seed_to_soil_maps.clone(), soil_to_fertilizer_maps.clone());
    let fertilizer_mapper = Mapper::new( soil_to_fertilizer_maps.clone(), fertilizer_to_water_maps.clone());
    let water_mapper = Mapper::new(fertilizer_to_water_maps.clone(), water_to_light_maps.clone());
    let light_mapper = Mapper::new(water_to_light_maps.clone(), light_to_temperature_maps.clone());
    let temperature_mapper = Mapper::new(light_to_temperature_maps.clone(), temperature_to_humidity_maps.clone());
    let humidity_mapper = Mapper::new(temperature_to_humidity_maps.clone(), humidity_to_location_maps.clone());
    let location_mapper = Mapper::new(humidity_to_location_maps.clone(), vec![]);

    let mut min_seed_location: (u64, u64) = (u64::MAX, u64::MAX);

    for seed in seeds {
      let soil = seed_mapper.next(seed);
      let fertilizer = soil_mapper.next(soil);
      let water = fertilizer_mapper.next(fertilizer);
      let light = water_mapper.next(water);
      let temperature = light_mapper.next(light);
      let humidity = temperature_mapper.next(temperature);
      let location = humidity_mapper.next(humidity);

      if location < min_seed_location.1 {
        min_seed_location = (seed, location);
      }      
    }

    println!("Day 5 Part 1: {}", min_seed_location.1);

}
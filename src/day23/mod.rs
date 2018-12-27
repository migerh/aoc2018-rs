use std::str::FromStr;
use super::utils::{ParseError, Error};
use regex::Regex;
use std::fmt::{Formatter, Display};

type Position = (f64, f64, f64);

#[derive(Debug, Clone)]
struct NanoBot {
  pub pos: Position,
  pub radius: f64,
}

impl Display for NanoBot {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      write!(f, "Pos: ({}, {}, {}), Radius: {}", self.pos.0, self.pos.1, self.pos.2, self.radius)
  }
}

impl FromStr for NanoBot {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<NanoBot, ParseError> {
    lazy_static!{
      static ref RE_bot: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    }

    let capture = match RE_bot.captures(s) {
      Some(c) => c,
      None => Err(ParseError::new("Could not parse bot"))?
    };

    let pos = (capture[1].parse::<f64>()?, capture[2].parse::<f64>()?, capture[3].parse::<f64>()?);
    let radius = capture[4].parse::<f64>()?;

    Ok(NanoBot { pos, radius })
  }
}

fn manhattan_distance(p: Position, q: Position) -> f64 {
  (p.0 - q.0).abs() + (p.1 - q.1).abs() + (p.2 - q.2).abs()
}

fn load_bots() -> Result<Vec<NanoBot>, ParseError> {
  let input = include_str!("./data/input.txt");

  input
    .split("\n")
    .map(|s| NanoBot::from_str(s))
    .collect::<Result<Vec<NanoBot>, ParseError>>()
}

pub fn problem1() -> Result<(), Error> {
  let bots = load_bots()?;
  let mut strongest = 0;
  let mut strongest_signal = 0f64;
  for (index, bot) in bots.iter().enumerate() {
    if bot.radius > strongest_signal {
      strongest_signal = bot.radius;
      strongest = index;
    }
  }

  println!("Strongest bot is {:?}", bots[strongest]);
  let strongest_bot = &bots[strongest];

  let number_in_reach = bots
    .iter()
    .filter(|b| {
      manhattan_distance(b.pos, strongest_bot.pos.clone()) <= strongest_bot.radius
    })
    .count();

  println!("Strongest bot can reach {} other bots", number_in_reach);

  Ok(())
}

fn scale_bots(bots: &Vec<NanoBot>, scale: f64) -> Vec<NanoBot> {
  let mut scaled_bots = bots.clone();

  for bot in scaled_bots.iter_mut() {
    bot.pos.0 /= scale;
    bot.pos.1 /= scale;
    bot.pos.2 /= scale;
    bot.radius /= scale;
  }

  scaled_bots
}

fn find_bots_in_range(bots: &Vec<NanoBot>, p: &Position) -> usize {
  let mut in_range = 0;
  for b in bots {
    if manhattan_distance(b.pos, *p) < b.radius + 1f64 {
      in_range += 1;
    }
  }

  in_range
}

fn find_intersection(bots: &Vec<NanoBot>, seed: Position, scale: f64) -> Position {
  let scaled_bots = scale_bots(bots, scale);
  let buffer = 16;
  let from = -buffer;
  let to = buffer;

  let mut max_intersect = 0;
  let mut closest_point = (std::f64::MAX, std::f64::MAX, std::f64::MAX);
  let mut closest_manhattan = std::f64::MAX;
  for z in from..to {
    for y in from..to {
      for x in from..to {
        let p = (seed.0 + x as f64, seed.1 + y as f64, seed.2 + z as f64);

        let bots_in_range = find_bots_in_range(&scaled_bots, &p);

        let manhattan = p.0.abs() + p.1.abs() + p.2.abs();
        let better_distance = bots_in_range == max_intersect && manhattan < closest_manhattan;
        if bots_in_range > max_intersect || better_distance {
          max_intersect = bots_in_range;

          closest_manhattan = manhattan;
          closest_point = p;
        }
      }
    }
  }

  println!("Max intersection: {}", max_intersect);
  println!("Closest point: {:?} (distance: {})", closest_point, closest_manhattan);

  closest_point
}

pub fn problem2() -> Result<(), Error> {
  let bots = load_bots()?;

  let mut seed = (0f64, 0f64, 0f64);
  // for i in 0..24 {
  for i in 0..8 {
    // let scale = 2f64.powf(23f64-i as f64);
    let scale = 10f64.powf(7f64-i as f64);
    println!("seeds: {:?}, scale: {}", seed, scale);

    seed = find_intersection(&bots, seed, scale);

    // seed.0 *= 2f64; //10f64;
    // seed.1 *= 2f64; //10f64;
    // seed.2 *= 2f64; //10f64;
    seed.0 *= 10f64;
    seed.1 *= 10f64;
    seed.2 *= 10f64;
  }
  // seed = find_intersection(&bots, seed, 1_000_000);
  // println!("Seed: {}, distance: {}", seeds.len(), manhattan_distance((0f64, 0f64, 0f64), seed));
  

  Ok(())
}
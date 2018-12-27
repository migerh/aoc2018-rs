use std::str::FromStr;
use super::utils::{ParseError, Error};
use regex::Regex;
use std::collections::HashMap;
use std::cmp::max;

type Position = (i64, i64, i64);

#[derive(Debug, Clone)]
struct NanoBot {
  pub pos: Position,
  pub radius: u64,
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

    let pos = (capture[1].parse::<i64>()?, capture[2].parse::<i64>()?, capture[3].parse::<i64>()?);
    let radius = capture[4].parse::<u64>()?;

    Ok(NanoBot { pos, radius })
  }
}

fn manhattan_distance(p: Position, q: Position) -> u64 {
  (p.0 - q.0).abs() as u64 + (p.1 - q.1).abs() as u64 + (p.2 - q.2).abs() as u64
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
  let mut strongest_signal = 0;
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

fn scale_bots(bots: &Vec<NanoBot>, scale: i64) -> Vec<NanoBot> {
  let mut scaled_bots = bots.clone();

  for bot in scaled_bots.iter_mut() {
    bot.pos.0 /= scale;
    bot.pos.1 /= scale;
    bot.pos.2 /= scale;
    bot.radius /= scale as u64;
  }

  scaled_bots
}

fn find_bots_in_range(bots: &Vec<NanoBot>, p: &Position) -> usize {
  let mut in_range = 0;
  for b in bots {
    if manhattan_distance(b.pos, *p) <= b.radius + 1 {
      in_range += 1;
    }
  }

  in_range
}

fn find_intersection(bots: &Vec<NanoBot>, seed: Position, scale: i64) -> Position {
  let scaled_bots = scale_bots(bots, scale);
  // let from = -120_000_000 / scale;
  // let to = 120_000_000 / scale;
  let buffer = 40;
  let from = -buffer;
  let to = buffer;

  let mut max_intersect = 0;
  let mut closest_point = (std::i64::MAX, std::i64::MAX, std::i64::MAX);
  let mut closest_manhattan = std::u64::MAX;
  for z in from..to {
    for y in from..to {
      for x in from..to {
        let p = (seed.0 + x, seed.1 + y, seed.2 + z);

        let bots_in_range = find_bots_in_range(&scaled_bots, &p);

        if bots_in_range >= max_intersect {
          max_intersect = bots_in_range;

          let manhattan = manhattan_distance((0, 0, 0), p);
          if manhattan < closest_manhattan {
            closest_manhattan = manhattan;
            closest_point = p;
          }
        }
      }
    }
    // println!("Layer {} of {}", z, to-from);
  }

  println!("Max intersection: {}", max_intersect);
  println!("Closest point: {:?} (distance: {})", closest_point, closest_manhattan);

  closest_point
}

pub fn problem2() -> Result<(), Error> {
  let bots = load_bots()?;

  let mut seed = (0, 0, 0);
  // for i in 0..23 {
  for i in 0..7 {
    let scale = 10i64.pow(6-i);
    println!("seed: {:?}, scale: {}", seed, scale);

    seed = find_intersection(&bots, seed, scale);
    seed.0 *= 10;
    seed.1 *= 10;
    seed.2 *= 10;
  }
  // seed = find_intersection(&bots, seed, 1_000_000);
  println!("Seed: {:?}, distance: {}", seed, manhattan_distance((0, 0, 0), seed));

  Ok(())
}
use std::str::FromStr;
use super::utils::{ParseError, Error};
use regex::Regex;

type Position = (i64, i64, i64);

#[derive(Debug)]
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

pub fn problem1() -> Result<(), Error> {
  let input = include_str!("./data/input.txt");
  let bots = input
    .split("\n")
    .map(|s| NanoBot::from_str(s))
    .collect::<Result<Vec<NanoBot>, ParseError>>()?;

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
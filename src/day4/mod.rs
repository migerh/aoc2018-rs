use std::num::ParseIntError;
use std::str::FromStr;
use regex::Regex;
use std::collections::BTreeMap;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
  pub what: String
}

impl ParseError {
  fn new(s: &str) -> ParseError {
    let what = s.to_string();
    ParseError { what }
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.what)
  }
}

impl Error for ParseError {
  fn description(&self) -> &str {
    "Error while parsing input"
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}

impl From<ParseIntError> for ParseError {
  fn from(_error: ParseIntError) -> Self {
    ParseError::new("Unable to parse integer")
  }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Action {
  Starts(u32),
  FallsAsleep,
  WakesUp
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Timestamp {
  pub year: u32,
  pub month: u32,
  pub day: u32,
  pub hour: u32,
  pub minute: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Record {
  pub time: Timestamp,
  pub action: Action,
}

impl FromStr for Action {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static!{
      static ref REFallsAsleep: Regex = Regex::new(r"falls asleep").unwrap();
      static ref REWakesUp: Regex = Regex::new(r"wakes up").unwrap();
      static ref REStarts: Regex = Regex::new(r"Guard \#(\d+) begins shift").unwrap();
    }

    if REFallsAsleep.is_match(s) {
      return Ok(Action::FallsAsleep);
    }

    if REWakesUp.is_match(s) {
      return Ok(Action::WakesUp);
    }

    if REStarts.is_match(s) {
      let cap = match REStarts.captures(s) {
        Some(capture) => capture,
        None => Err(ParseError::new("Could not parse action"))?
      };
      let guard_id = cap[1].parse::<u32>()?;

      return Ok(Action::Starts(guard_id));
    }

    Err(ParseError::new("Could not parse action"))
  }
}

impl FromStr for Timestamp {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static!{
      static ref RE: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)").unwrap();
    }
    let cap = RE.captures(s).unwrap();

    let parse = |v: &str| v.parse::<u32>();

    let year = parse(&cap[1])?;
    let month = parse(&cap[2])?;
    let day = parse(&cap[3])?;
    let hour = parse(&cap[4])?;
    let minute = parse(&cap[5])?;

    Ok(Timestamp { day, month, year, hour, minute })
  }
}

impl FromStr for Record {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let time = Timestamp::from_str(s)?;
    let action = Action::from_str(s)?;

    Ok(Record { time, action })
  }
}

fn parse_and_sort() -> Result<Vec<Record>, ParseError> {
  let input = include_str!("./data/input.txt");

  let mut records = input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| Record::from_str(v))
    .collect::<Result<Vec<Record>, _>>()?;

  records.sort();

  Ok(records)
}

fn get_sleeping_patterns(records: &Vec<Record>) -> BTreeMap<(u32, u32), u32> {
  let mut map = BTreeMap::new();
  let mut current_guard: u32 = 0;
  let mut falls_asleep_minute = 0;

  for record in records {
    match record.action {
      Action::Starts(guard) => current_guard = guard,
      Action::FallsAsleep => falls_asleep_minute = record.time.minute,
      Action::WakesUp => {
        for i in falls_asleep_minute..record.time.minute {
          map
            .entry((current_guard, i))
            .and_modify(|v| *v += 1)
            .or_insert(1);
        }
      }
    }
  }

  map
}

pub fn problem1() -> Result<u32, ParseError> {
  let records = parse_and_sort()?;
  let sleeping_patterns = get_sleeping_patterns(&records);

  let mut total_minutes_per_guard: BTreeMap<u32, u32> = BTreeMap::new();
  for (key, value) in &sleeping_patterns {
    total_minutes_per_guard
      .entry(key.0)
      .and_modify(|v: &mut u32| *v = *v + *value)
      .or_insert(*value);
  }

  let mut guard_who_slept_most = 0;
  let mut minutes_that_guard_slept = 0;

  for guard in total_minutes_per_guard {
    if guard.1 > minutes_that_guard_slept {
      guard_who_slept_most = guard.0;
      minutes_that_guard_slept = guard.1;
    }
  }
  println!("Guard {} slept a total of {} minutes", guard_who_slept_most, minutes_that_guard_slept);

  let mut minute_that_guard_slept_most = 0;
  let mut number_of_times_that_guard_slept_in_that_minute = 0;
  for (key, value) in &sleeping_patterns {
    if key.0 == guard_who_slept_most && number_of_times_that_guard_slept_in_that_minute < *value {
      minute_that_guard_slept_most = key.1;
      number_of_times_that_guard_slept_in_that_minute = *value;
    }
  }

  println!("And he slept the most in minute {}", minute_that_guard_slept_most);
  let result = guard_who_slept_most * minute_that_guard_slept_most;
  println!("Solution: {}", result);

  Ok(result)
}

pub fn problem2() -> Result<u32, ParseError> {
  let records = parse_and_sort()?;
  let sleeping_patterns = get_sleeping_patterns(&records);

  let mut guard = 0;
  let mut total_minutes = 0;
  let mut minute = 0;
  for (key, value) in &sleeping_patterns {
    if *value > total_minutes {
      guard = key.0;
      total_minutes = *value;
      minute = key.1;
    }
  }

  let result = guard * minute;
  println!("Solution: {}", result);

  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_problem1_result() {
    assert_eq!(problem1().unwrap(), 103720);
  }

  #[test]
  fn check_problem2_result() {
    assert_eq!(problem2().unwrap(), 110913);
  }
}
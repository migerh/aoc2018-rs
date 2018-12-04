use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;

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

#[derive(Debug, Eq)]
struct Record {
  pub time: Timestamp,
  pub action: Action,
}

impl FromStr for Timestamp {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static!{
      static ref RE: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)").unwrap();
    }
    let cap = RE.captures(s).unwrap();

    let parse = |v: &str| v.parse::<u32>().unwrap();

    let year = parse(&cap[1]);
    let month = parse(&cap[2]);
    let day = parse(&cap[3]);
    let hour = parse(&cap[4]);
    let minute = parse(&cap[5]);

    Ok(Timestamp { day, month, year, hour, minute })
  }
}

impl FromStr for Record {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let time = Timestamp::from_str(s)?;

    lazy_static!{
      static ref REFallsAsleep: Regex = Regex::new(r"falls asleep").unwrap();
      static ref REWakesUp: Regex = Regex::new(r"wakes up").unwrap();
      static ref REStarts: Regex = Regex::new(r"Guard \#(\d+) begins shift").unwrap();
    }

    let mut action = Action::WakesUp;

    if REFallsAsleep.is_match(s) {
      action = Action::FallsAsleep;
    }

    if REWakesUp.is_match(s) {
      action = Action::WakesUp;
    }

    if REStarts.is_match(s) {
      let cap = REStarts.captures(s).unwrap();
      let parse = |v: &str| v.parse::<u32>().unwrap();

      action = Action::Starts(parse(&cap[1]));
    }

    Ok(Record { time, action })
  }
}

impl Ord for Record {
  fn cmp(&self, other: &Record) -> Ordering {
    self.time.cmp(&other.time)
  }
}

impl PartialOrd for Record {
  fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Record {
  fn eq(&self, other: &Record) -> bool {
    self.time == other.time
  }
}

fn parse_and_sort() -> Vec<Record> {
  let input = include_str!("./data/input.txt");

  let mut records = input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| match Record::from_str(v) {
      Ok(v) => v,
      _ => panic!("Parse error")
    })
    .collect::<Vec<_>>();

  records.sort();

  records
}

pub fn problem1() {
  let records = parse_and_sort();
  let mut map = BTreeMap::new();
  let mut current_guard: u32 = 0;
  let mut falls_asleep_minute = 0;

  for record in &records {
    match record.action {
      Action::Starts(guard) => current_guard = guard,
      Action::FallsAsleep => falls_asleep_minute = record.time.minute,
      Action::WakesUp => {
        for i in falls_asleep_minute..record.time.minute {
          map.insert((current_guard, i), true);
        }
      }
    }
  }

  println!("{:?}", records);
}
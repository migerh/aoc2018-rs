use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Box {
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

impl FromStr for Box {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split("@");
    split.next();
    let mut data = match split.next() {
      Some(v) => v.split(":"),
      None => panic!("Parse error")
    };

    let coordinates = match data.next() {
      Some(v) => v.trim(),
      None => panic!("Parse error")
    };
    let dimension = match data.next() {
      Some(v) => v.trim(),
      None => panic!("Parse error")
    };

    let mut position_split = coordinates.split(",");
    let x = match position_split.next() {
      Some(v) => v.parse::<u32>().unwrap(),
      None => panic!("Parse error")
    };
    let y = match position_split.next() {
      Some(v) => v.parse::<u32>().unwrap(),
      None => panic!("Parse error")
    };

    let mut dimension_split = dimension.split("x");
    let w = match dimension_split.next() {
      Some(v) => v.parse::<u32>().unwrap(),
      None => panic!("Parse error")
    };
    let h = match dimension_split.next() {
      Some(v) => v.parse::<u32>().unwrap(),
      None => panic!("Parse error")
    };

    Ok(Box { x, y, w, h })
  }
}

pub fn problem1() {
  let input = include_str!("./data/input.txt");
  let boxes = input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| Box::from_str(v))
    .map(|v| match v {
      Ok(b) => b,
      _ => panic!("Parse error"),
    });

  let mut map = BTreeMap::new();

  for b in boxes {
    for i in 0..b.w {
      for k in 0..b.h {
        let x = b.x + i;
        let y = b.y + k;

        map.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
      }
    }
  }

  let mut count = 0;
  for square in map {
    if square.1 > 1 {
      count += 1
    }
  }
  println!("Number of overlapping squares {}", count);
}

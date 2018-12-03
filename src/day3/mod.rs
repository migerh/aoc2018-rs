use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Box {
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
  pub id: u32,
}

impl FromStr for Box {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split("@");
    let id_content = match split.next() {
      Some(v) => v,
      None => panic!("Parse error")
    };

    let mut id_number = id_content.split("#");
    id_number.next();
    let id = match id_number.next() {
      Some(v) => v.trim().parse::<u32>().unwrap(),
      None => panic!("Parse error")
    };

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

    Ok(Box { id, x, y, w, h })
  }
}

fn parse_boxes() -> Vec<Box> {
  let input = include_str!("./data/input.txt");
  let boxes = input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| Box::from_str(v))
    .map(|v| match v {
      Ok(b) => b,
      _ => panic!("Parse error"),
    })
    .collect::<Vec<_>>();

  boxes
}

pub fn problem1() -> BTreeMap<u32, bool> {
  let boxes = parse_boxes();
  let mut map = BTreeMap::new();
  let mut doubles = BTreeMap::new();

  for b in boxes {
    doubles.insert(b.id, true);

    for i in 0..b.w {
      for k in 0..b.h {
        let x = b.x + i;
        let y = b.y + k;

        map.entry((x, y)).and_modify(|v: & mut Vec<u32>| {
          v.push(b.id);
          for i in v {
            doubles.entry(*i).and_modify(|w| *w = false);
          }
        }).or_insert(vec![b.id]);
      }
    }
  }

  let mut count = 0;
  for square in &map {
    if square.1.len() > 1 {
      count += 1
    }
  }
  println!("Number of overlapping squares {}", count);

  doubles
}

pub fn problem2() {
  let doubles = problem1();

  for d in doubles {
    if d.1 == true {
      println!("Box with id {} is non-overlapping!", d.0);
    }
  }
}
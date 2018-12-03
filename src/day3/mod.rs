use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::BTreeMap;


use regex::Regex;

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
    lazy_static!{
      static ref RE: Regex = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let cap = RE.captures(s).unwrap();

    let parse = |v: &str| v.parse::<u32>().unwrap();

    let id = parse(&cap[1]);
    let x = parse(&cap[2]);
    let y = parse(&cap[3]);
    let w = parse(&cap[4]);
    let h = parse(&cap[5]);

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

fn squares_to_source_map() -> BTreeMap<(u32, u32), Vec<u32>> {
  let boxes = parse_boxes();
  let mut map = BTreeMap::new();

  for b in boxes {
    for i in 0..b.w {
      for k in 0..b.h {
        let x = b.x + i;
        let y = b.y + k;

        map.entry((x, y)).and_modify(|v: & mut Vec<u32>| {
          v.push(b.id);
        }).or_insert(vec![b.id]);
      }
    }
  }

  map
}

pub fn problem1() {
  let map = squares_to_source_map();

  let mut count = 0;
  for square in &map {
    if square.1.len() > 1 {
      count += 1
    }
  }

  println!("Number of overlapping squares {}", count);
}

pub fn problem2() {
  let map = squares_to_source_map();
  let mut doubles = BTreeMap::new();

  for d in map {
    let box_ids = d.1;

    if box_ids.len() == 1 {
      doubles
        .entry(box_ids[0])
        .and_modify(|v| *v = *v && true)
        .or_insert(true);
    }

    if box_ids.len() > 1 {
      for id in box_ids {
        doubles
          .entry(id)
          .and_modify(|v| *v = false)
          .or_insert(false);
      }
    }
  }

  for d in doubles {
    if d.1 == true {
      println!("Box with id {} is non-overlapping!", d.0);
    }
  }
}
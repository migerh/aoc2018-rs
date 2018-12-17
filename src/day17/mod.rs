use std::collections::HashMap;
use std::ops::Range;
use regex::Regex;
use std::cmp::{min, max};
use super::utils::{Error, ParseError};

type Position = (i32, i32);

struct Board {
  pub offset: Position,
  pub bbox: (Position, Position),
  pub map: Vec<Vec<char>>,
}

fn parse_range(regex: &Regex, line: &str) -> Result<Range<i32>, ParseError> {
  let capture = match regex.captures(line) {
    Some(c) => c,
    None => Err(ParseError::new("Could not parse line"))?
  };

  let start = match capture.get(1) {
    Some(v) => v.as_str().parse::<i32>()?,
    None => Err(ParseError::new("Could not parse start"))?
  };

  let range = match capture.get(3) {
    Some(v) => start..v.as_str().parse::<i32>()?,
    None => start..start+1
  };

  Ok(range)
}

fn parse_line(map: &mut Vec<Position>, line: &str) -> Result<(), ParseError> {
  lazy_static!{
    static ref RE_x: Regex = Regex::new(r"x=(\d+)(\.\.)?(\d+)?").unwrap();
    static ref RE_y: Regex = Regex::new(r"y=(\d+)(\.\.)?(\d+)?").unwrap();
  }

  let x_range = parse_range(&RE_x, line)?;
  let y_range = parse_range(&RE_y, line)?;

  for x in x_range.start..x_range.end {
    for y in y_range.start..y_range.end {
      map.push((x, y));
    }
  }

  Ok(())
}

fn bounding_box(positions: &Vec<Position>) -> (Position, Position) {
  let mut minp = (std::i32::MAX, std::i32::MAX);
  let mut maxp = (std::i32::MIN, std::i32::MIN);

  for pos in positions {
    minp.0 = min(pos.0, minp.0);
    minp.1 = min(pos.1, minp.1);
    maxp.0 = max(pos.0, maxp.0);
    maxp.1 = max(pos.1, maxp.1);
  }

  (minp, maxp)
}

fn initialize() -> Result<Board, Error> {
  let input = include_str!("./data/input.txt");
  let mut positions = vec![];

  for line in input.split('\n') {
    parse_line(&mut positions, line)?;
  }

  let bbox = bounding_box(&positions);
  let offset = ((bbox.0).0 - 1, 0);
  let size = ((bbox.1).0 - offset.0 + 1, (bbox.1).1 - offset.1);

  println!("The map goes from {:?} to {:?}, size is {:?}", bbox.0, bbox.1, size);

  let mut map = vec![vec!['.'; (size.0 + 1) as usize]; (size.1 + 1) as usize];

  for pos in positions {
    let p = (pos.0 - offset.0, pos.1 - offset.1);
    map[p.1 as usize][p.0 as usize] = '#';
  }

  let board = Board { offset, bbox, map };
  Ok(board)
}

fn print(board: &Board) {
  for line in &board.map {
    let s: String = line.iter().collect();
    println!("{}", s);
  }
}

pub fn problem1() -> Result<(), Error> {
  let board = initialize()?;
  print(&board);

  Ok(())
}
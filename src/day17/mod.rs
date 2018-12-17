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

impl Board {
  pub fn get(&self, pos: Position) -> char {
    let p = (pos.0 - self.offset.0, pos.1 - self.offset.1);
    self.map[p.1 as usize][p.0 as usize]
  }

  pub fn set(&mut self, pos: Position, c: char) {
    let p = (pos.0 - self.offset.0, pos.1 - self.offset.1);
    self.map[p.1 as usize][p.0 as usize] = c;
  }
}

static debug: bool = true;

fn log(s: String) {
  if debug {
    println!("{}", s);
  }
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
    Some(v) => start..v.as_str().parse::<i32>()?+1,
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
  let input = include_str!("./data/example.txt");
  let mut positions = vec![];

  for line in input.split('\n') {
    parse_line(&mut positions, line)?;
  }

  let bbox = bounding_box(&positions);
  // let offset = ((bbox.0).0 - 1, (bbox.0).1);
  let offset = ((bbox.0).0 - 1, 0);
  let size = ((bbox.1).0 - offset.0 + 1, (bbox.1).1 - offset.1);

  log(format!("The map goes from {:?} to {:?}, size is {:?}", bbox.0, bbox.1, size));

  let mut map = vec![vec!['.'; (size.0 + 1) as usize]; (size.1 + 1) as usize];

  for pos in positions {
    let p = (pos.0 - offset.0, pos.1 - offset.1);
    map[p.1 as usize][p.0 as usize] = '#';
  }

  let board = Board { offset, bbox, map };
  Ok(board)
}

fn print(board: &Board) {
  let offset = board.offset;
  let cols = board.map[0].len() as i32;
  for l in 0..3 {
    print!("    ");
    for x in 0..cols {
      let n = x + offset.0;
      let c = (n / (10 as i32).pow(2 - l)) % 10;
      print!("{}", c);
    }
    println!("");
  }

  for (row, line) in board.map.iter().enumerate() {
    let s: String = line.iter().collect();
    print!("{:04}", row);
    println!("{}", s);
  }
}

fn is_sand(c: char) -> bool {
  c == '.'
}

fn find_next_stop_down(seed: Position, mut board: &mut Board) -> Position {
  let (x, mut y) = seed;
  let max = board.map.len() as i32;

  y += 1;
  while y < max && is_sand(board.get((x, y))) {
    board.set((x, y), 'w');
    y += 1;
  }

  (x, y)
}

fn fill_bucket(seed: Position, board: &mut Board) -> Vec<Position> {
  let (x, y) = seed;

  let mut flow_left = true;
  let mut flow_right = true;
  let mut new_seeds = vec![];
  let width = board.offset.0 + (board.map[0].len() as i32);
  for dx in 1..width {
    if !flow_left && !flow_right {
      break;
    }

    log(format!("Flowing {} of {}", dx, width));

    if x - dx < 0 {
      flow_left = false;
    }

    if x + dx >= width {
      flow_right = false;
    }

    log(format!("Flow left, looking at {}, {}", x - dx, y));
    if flow_left && is_sand(board.get((x - dx, y + 1))) {
      log(format!("Left: Sand below"));
      new_seeds.push((x - dx, y));
      flow_left = false;
    }

    if flow_left && !is_sand(board.get((x - dx, y))) {
      log(format!("Left: Hit wall"));
      flow_left = false;
    }

    if flow_left {
      log(format!("Set {}, {}", x - dx, y));
      board.set((x - dx, y), 'w');
    }

    log(format!("Flow right, looking at {}, {}", x + dx, y));
    if flow_right && is_sand(board.get((x + dx, y + 1))) {
      log(format!("Right: Sand below"));
      new_seeds.push((x + dx, y));
      flow_right = false;
    }

    if flow_right && !is_sand(board.get((x + dx, y))) {
      log(format!("Right: Hit wall"));
      flow_right = false;
    }

    if flow_right {
      log(format!("Set {}, {}", x + dx, y));
      board.set((x + dx, y), 'w');
    }
  }

  log(format!("New seeds: {:?}", new_seeds));
  new_seeds
}

fn trace(seed: Position, mut board: &mut Board) {
  let mut seeds = vec![seed];
  let max_y = board.map.len() as i32;

  while !seeds.is_empty() {
    let next = seeds.pop().unwrap();
    board.set(next, 'w');
    let (x, mut y) = find_next_stop_down(next, &mut board);

    if y >= max_y {
      continue;
    }

    log(format!("Stop down: {:?}", (x, y)));

    y -= 1;
    let mut next_seeds = fill_bucket((x, y), &mut board);
    while next_seeds.is_empty() {
      y -= 1;
      log(format!("Fill {}, {}", x, y));
      next_seeds = fill_bucket((x, y), &mut board);
    }

    seeds.append(&mut next_seeds);
  }
}

fn count_water(board: &Board) -> i32 {
  let skip_rows = (board.bbox.0).1;
  // todo: calculate this instead of hard wiring it
  let skip_cols = 1;

  let mut count = 0;
  for row in board.map.iter().skip(skip_rows as usize) {
    for chr in row.iter().skip(skip_cols as usize) {
      if *chr == 'w' {
        count += 1;
      }
    }
  }

  count
}

pub fn problem1() -> Result<(), Error> {
  let mut board = initialize()?;
  print(&board);

  println!("Tracing water…");

  trace((500, 0), &mut board);
  print(&board);

  let result = count_water(&board);
  println!("Result: {}", result);

  Ok(())
}
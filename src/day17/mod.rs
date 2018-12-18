use std::ops::Range;
use regex::Regex;
use std::cmp::{min, max};
use super::utils::{Error, ParseError};

type Position = (i32, i32);

struct Board {
  pub offset: Position,
  pub bbox: (Position, Position),
  pub size: Position,
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

  pub fn is_sand(&self, pos: Position) -> bool {
    self.get(pos) == '.'
  }

  pub fn is_clay(&self, pos: Position) -> bool {
    self.get(pos) == '#'
  }

  pub fn is_settled_water(&self, pos: Position) -> bool {
    self.get(pos) == '~'
  }

  pub fn is_moving_water(&self, pos: Position) -> bool {
    self.get(pos) == '|'
  }

  pub fn can_flow(&self, pos: Position) -> bool {
    self.is_sand(pos) || self.is_moving_water(pos)
  }
}

static DEBUG: bool = false;
static ROWS: i32 = 2000;

fn log(s: String) {
  if DEBUG {
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
  let input = include_str!("./data/input.txt");
  let mut positions = vec![];

  for line in input.split('\n') {
    parse_line(&mut positions, line)?;
  }

  let bbox = bounding_box(&positions);
  let (min_point, max_point) = bbox;
  // let offset = ((bbox.0).0 - 1, (bbox.0).1);
  let offset = (min_point.0 - 3, 0);
  // let size = ((bbox.1).0 - offset.0 + 1, (bbox.1).1 - offset.1);
  let size = (max_point.0 - min_point.0, max_point.1 - min_point.1);

  //log(format!("The map goes from {:?} to {:?}, size is {:?}", bbox.0, bbox.1, size));

  let mut map = vec![vec!['.'; (size.0 + 6) as usize]; (size.1 + 15) as usize];

  for pos in positions {
    if pos.1 > ROWS {
      continue;
    }

    let p = (pos.0 - offset.0, pos.1 - offset.1);
    map[p.1 as usize][p.0 as usize] = '#';
  }

  let board = Board { offset, bbox, map, size };
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

  for (row, line) in board.map.iter().take((ROWS+3) as usize).enumerate() {
    let s: String = line.iter().collect();
    print!("{:04}", row);
    println!("{}", s);
  }
}

fn is_falling_water(c: char) -> bool {
  c == '|'
}

fn is_settled_water(c: char) -> bool {
  c == '~'
}

fn is_water(c: char) -> bool {
  is_falling_water(c) || is_settled_water(c)
}

// An edge is something like this:
// flowing left:    flowing right:
//
//  E~~~                 ~~~E
//   #~~                 ~~#
//
// -> We have clay at (E.x - dir, E.y + 1),
// water at (E.x - dir, E.y) and sand at
// (E.x, E.y + 1)
fn is_free_edge(dir: i32, p: Position, board: &Board) -> bool {
  let (ex, ey) = p;
  let clay = (ex - dir, ey + 1);
  let water = (ex - dir, ey);
  let sand = (ex, ey + 1);

  let result = board.can_flow(p) &&
  board.can_flow(water) &&
  board.can_flow(sand) &&
  !board.can_flow(clay);

  result
}

fn find_next_stop_down(seed: Position, board: &mut Board) -> Option<Position> {
  let (x, mut y) = seed;
  let maxh = board.map.len() as i32;

  y += 1;
  while y < maxh && board.can_flow((x, y)) {
    board.set((x, y), '|');
    y += 1;
  }

  if y == maxh {
    return None;
  }

  Some((x, y))
}

fn fill_bucket(seed: Position, board: &mut Board) -> Vec<Position> {
  let (x, y) = seed;
  let fill = '|';

  let mut flow_left = true;
  let mut flow_right = true;
  let mut new_seeds = vec![];
  let width = board.offset.0 + (board.map[0].len() as i32);
  for dx in 0..width {
    if !flow_left && !flow_right {
      break;
    }

    if x - dx < board.offset.0 {
      flow_left = false;
    }

    if x + dx >= width {
      flow_right = false;
    }

    //log(format!("Flow left, looking at {}, {}", x - dx, y));
    // if flow_left && is_sand_or_water(board.get((x - dx, y + 1))) && is_sand(board.get((x - dx, y))) {
    if flow_left && is_free_edge(-1, (x - dx, y), &board) {
      //log(format!("Left: Sand below"));
      new_seeds.push((x - dx, y));
      flow_left = false;
    }

    // if flow_left && is_occupied_edge(-1, (x - dx, y), &board) {
    //   log(format!("Left: Hit wall"));
    //   flow_left = false;
    // }

    if flow_left && board.is_clay((x - dx, y)) {
      //log(format!("Left: Hit wall"));
      flow_left = false;
    }

    if flow_left {
      //log(format!("Set {}, {}", x - dx, y));
      board.set((x - dx, y), fill);
    }

    //log(format!("Flow right, looking at {}, {}", x + dx, y));
    // if flow_right && is_sand_or_falling_water(board.get((x + dx, y + 1))) && is_sand(board.get((x + dx, y))) {
    if flow_right && is_free_edge(1, (x + dx, y), &board) {
      //log(format!("Right: Sand below"));
      new_seeds.push((x + dx, y));
      flow_right = false;
    }

    // if flow_right && is_occupied_edge(1, (x + dx, y), &board) {
    //   log(format!("Right: Sand below"));
    //   flow_right = false;
    // }

    if flow_right && board.is_clay((x + dx, y)) {
      //log(format!("Right: Hit wall"));
      flow_right = false;
    }

    if flow_right {
      //log(format!("Set {}, {}", x + dx, y));
      board.set((x + dx, y), fill);
    }
  }

  //log(format!("New seeds: {:?}", new_seeds));
  new_seeds
}

fn settle_water(y: i32, board: &mut Board) -> bool {
  let start = board.offset.0;
  let end = start + board.map[0].len() as i32;
  let mut result = false;

  // new strategy here:
  // to determine whether water can be settled, the flowing
  // water in a row has to be clustered and the water is only
  // settled, if and only if the whole cluster can be settled.

  let mut clusters = vec![];
  let mut current = vec![];
  let mut previous_was_flowing = false;
  for x in start..end {
    if board.is_moving_water((x, y)) {
      current.push((x, y));
      previous_was_flowing = true;
    } else if previous_was_flowing {
      clusters.push(current);
      current = vec![];
      previous_was_flowing = false;
    }
  }

  if !current.is_empty() {
    clusters.push(current);
  }

  for cluster in clusters {
    let mut can_be_settled = true;

    if let Some(pos) = cluster.first() {
      if pos.0 - 1 < 0 {
        can_be_settled = false;
      } else if board.can_flow((pos.0 - 1, pos.1)) {
        can_be_settled = false;
      }
    }

    if let Some(pos) = cluster.last() {
      if board.can_flow((pos.0 + 1, pos.1)) {
        can_be_settled = false;
      }
    }

    for pos in &cluster {
      if board.can_flow((pos.0, pos.1 + 1)) {
        can_be_settled = false;
      }
    }

    if can_be_settled {
      for pos in &cluster {
        board.set(*pos, '~');
        result = true;
      }
    }
  }

  result
}

fn trace(seed: Position, mut board: &mut Board) {
  let mut seeds = vec![seed];
  let mut previous_seeds = vec![];

  while let Some(next) = seeds.pop() {
    previous_seeds.push(next.clone());

    board.set(next, '|');
    let (x, mut y) = match find_next_stop_down(next, &mut board) {
      Some(v) => v,
      None => continue
    };

    y -= 1;
    let mut next_seeds = fill_bucket((x, y), &mut board);
    while y > 0 && settle_water(y, &mut board) && next_seeds.is_empty() {
      y -= 1;
      next_seeds = fill_bucket((x, y), &mut board);
    }

    seeds.append(&mut next_seeds);

    if false {
      print(&board);
    }
  }
}

fn count_water(board: &Board) -> i32 {
  let skip_rows = (board.bbox.0).1;
  let size = board.size;
  // todo: calculate this instead of hard wiring it
  // let (min_point, _) = board.bbox;
  // let skip_cols = min_point.0 - board.offset.0;

  println!("Counting water. Skipping {} rows.", skip_rows);

  let mut count = 0;
  for row in board.map.iter().skip(skip_rows as usize).take((size.1 + 1) as usize) {
    for chr in row.iter() {
      if is_water(*chr) {
        count += 1;
      }
    }
  }

  count
}

pub fn problem1() -> Result<(), Error> {
  let mut board = initialize()?;
  // print(&board);

  println!("Board initialized: {:?}", board.bbox);
  println!("Tracing water…");

  trace((500, 0), &mut board);
  print(&board);

  let result = count_water(&board);
  println!("Result: {}", result);

  Ok(())
}
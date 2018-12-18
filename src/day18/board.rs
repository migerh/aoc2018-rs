use std::str::FromStr;
use super::super::utils::ParseError;

static DEBUG: bool = false;
fn log(s: String) {
  if DEBUG {
    println!("{}", s);
  }
}

#[derive(Debug)]
pub struct Stats {
  pub grows_tree: bool,
  pub constructs_lumberyard: bool,
  pub stays_lumberyard: bool,
}

pub struct Board {
  pub map: Vec<Vec<char>>,
  pub ticks: usize,
}

impl Board {
  pub fn checksum(&self) -> usize {
    let mut lumberyards = 0;
    let mut trees = 0;

    for line in &self.map {
      for c in line {
        match *c {
          '|' => trees += 1,
          '#' => lumberyards += 1,
          _ => {}
        }
      }
    }

    trees * lumberyards
  }

  fn analyze_neighbors(&self, pos: (usize, usize)) -> Stats {
    let mut trees = 0;
    let mut lumberyards = 0;

    let (x, y) = pos;
    let x = x as i32;
    let y = y as i32;

    for dy in -1i32..2i32 {
      for dx in -1i32..2i32 {
        if dy == 0 && dx == 0 {
          continue;
        }

        if x + dx < 0 || x + dx >= self.map[0].len() as i32 {
          continue;
        }

        if y + dy < 0 || y + dy >= self.map.len() as i32 {
          continue;
        }

        match self.map[(y + dy) as usize][(x + dx) as usize] {
          '|' => trees += 1,
          '#' => lumberyards += 1,
          _ => {}
        };
      }
    }

    log(format!("Stats debug -- pos: {:?}, trees: {}, lumberyards: {}", pos, trees, lumberyards));

    Stats {
      grows_tree: trees >= 3,
      constructs_lumberyard: lumberyards >= 3,
      stays_lumberyard: trees >= 1 && lumberyards >= 1,
    }
  }

  pub fn tick(&mut self) {
    self.ticks += 1;
    let mut next_map = self.map.clone();

    for (y, line) in self.map.iter().enumerate() {
      log(format!("Line {}", y));
      for (x, c) in line.iter().enumerate() {
        let stats = self.analyze_neighbors((x, y));
        log(format!("Debug -- now: {}, stats: {:?}", *c, stats));
        next_map[y][x] = match *c {
          '.' => if stats.grows_tree { '|' } else { '.' },
          '|' => if stats.constructs_lumberyard { '#' } else { '|' },
          '#' => if stats.stays_lumberyard { '#' } else { '.' },
          v => v
        };
      }
    }

    self.map = next_map;
  }

  pub fn debug(&self) {
    println!("");
    println!("After {} minutes", self.ticks);
    for line in &self.map {
      let s: String = line.iter().collect();
      println!("{}", s);
    }
  }
}

impl FromStr for Board {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Board, ParseError> {
    let map = s.split('\n').map(|v| v.chars().collect()).collect();
    let ticks = 0;

    Ok(Board { map, ticks })
  }
}
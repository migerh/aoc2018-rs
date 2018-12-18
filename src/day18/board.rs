use std::str::FromStr;
use super::super::utils::ParseError;

#[derive(Debug, Clone)]
pub struct Stats {
  pub grows_tree: bool,
  pub constructs_lumberyard: bool,
  pub stays_lumberyard: bool,
}

impl Stats {
  pub fn new() -> Stats {
    Stats { grows_tree: false, constructs_lumberyard: false, stays_lumberyard: false }
  }
}

pub struct Board {
  pub map: Vec<Vec<char>>,
  pub stats_map: Vec<Vec<Stats>>,
  pub ticks: usize,
  pub size: (usize, usize),
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

        match self.map[(y + dy) as usize][(x + dx) as usize] {
          '|' => trees += 1,
          '#' => lumberyards += 1,
          _ => {}
        };

        if trees >= 3 && lumberyards >= 3 {
          break;
        }
      }
    }

    Stats {
      grows_tree: trees >= 3,
      constructs_lumberyard: lumberyards >= 3,
      stays_lumberyard: trees >= 1 && lumberyards >= 1,
    }
  }

  fn update_stats(&mut self) {
    for (y, line) in self.map.iter().skip(1).take(self.size.1).enumerate() {
      for (x, _) in line.iter().skip(1).take(self.size.0).enumerate() {
        self.stats_map[y][x] = self.analyze_neighbors((x + 1, y + 1));
      }
    }
  }

  pub fn tick(&mut self) {
    self.ticks += 1;
    self.update_stats();

    for y in 1..self.map.len()-1 {
      for x in 1..self.map[y].len()-1 {
        let stats = &self.stats_map[y-1][x-1];
        let c = self.map[y][x];
        self.map[y][x] = match c {
          '.' => if stats.grows_tree { '|' } else { '.' },
          '|' => if stats.constructs_lumberyard { '#' } else { '|' },
          '#' => if stats.stays_lumberyard { '#' } else { '.' },
          v => v
        };
      }
    }
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
    let raw_map: Vec<Vec<char>> = s.split('\n').map(|v| v.chars().collect()).collect();
    let ticks = 0;
    let size = (raw_map[0].len(), raw_map.len());
    let stats_map = vec![vec![Stats::new(); size.0]; size.1];

    let mut map = vec![vec!['.'; size.0 + 2]; size.1 + 2];
    for y in 0..size.1 {
      for x in 0..size.0 {
        map[y + 1][x + 1] = raw_map[y][x];
      }
    }

    Ok(Board { map, stats_map, ticks, size })
  }
}
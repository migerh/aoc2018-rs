use std::collections::BTreeMap;
use std::str::FromStr;
use std::cmp::max;
use super::unit::{Unit, Kind, Position};
use super::super::utils::ParseError;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Tile {
  Floor,
  Wall,
}

pub type Board = BTreeMap<Position, Tile>;

pub struct Cave {
  pub board: Board,
  pub units: Vec<Unit>,
  pub dimension: (usize, usize),
  pub debug: bool,
}

fn print_board(board: &Board, (width, height): (usize, usize)) -> Vec<Vec<char>> {
  let mut out = vec![vec![' '; width]; height];

  for (pos, tile) in board {
    out[pos.1][pos.0] = match tile {
      Tile::Floor => '.',
      Tile::Wall => '#',
    };
  }

  out
}

fn print_units_onto_board(printed_board: &mut Vec<Vec<char>>, units: &Vec<Unit>) {
  for unit in units {
    let (x, y) = unit.position;
    printed_board[y][x] = match unit.kind {
      Kind::Elf => 'E',
      Kind::Goblin => 'G'
    };
  }
}

fn print_state_with_stats(state: &Vec<Vec<char>>, units: &Vec<Unit>) {
  for (index, line) in state.iter().enumerate() {
    let s = line.iter().collect::<String>();
    print!("{}  ", s);
    for unit in units.iter().filter(|v| v.position.1 == index) {
      let code = match unit.kind {
        Kind::Goblin => 'G',
        Kind::Elf => 'E',
      };
      print!("{}@{} ({}) ", code, unit.position.0, unit.health);
    }
    println!("");
  }
}

impl Cave {
  pub fn burn_units_in(&self, exclude: Position) -> Board {
    let mut board = self.board.clone();

    for unit in &self.units {
      if unit.position != exclude {
        board.entry(unit.position).and_modify(|v| *v = Tile::Wall);
      }
    }

    board
  }

  pub fn tick(&mut self) {
    let (width, height) = self.dimension;
    let mut updated = vec![false; self.units.len()];

    for y in 0..height {
      for x in 0..width {
        for (index, unit) in self.units.clone().iter().enumerate() {
          if unit.position == (x, y) && !updated[index] {
            updated[index] = true;
            self.units[index].position = match unit.move_unit(&self) {
              Some(v) => v,
              None => unit.position,
            };

            let (target_index, damage) = match self.units[index].attack(&self) {
              Some(v) => v,
              None => continue
            };

            self.units[target_index].health -= damage;
            if self.units[target_index].health <= 0 {
              self.units.remove(target_index);
              updated.remove(target_index);
            }

            break;
          }
        }
      }
    }
  }

  pub fn print_with_units(&self) {
    let mut out = print_board(&self.board, self.dimension);
    print_units_onto_board(&mut out, &self.units);
    print_state_with_stats(&out, &self.units);
  }

  fn load_board(input: &str) -> (Board, (usize, usize)) {
    let mut result = BTreeMap::new();
    let mut max_col = 0;
    let mut max_row = 0;

    for (row, line) in input.split('\n').enumerate() {
      max_row = max(max_row, row);
      for (column, chr) in line.chars().enumerate() {
        max_col = max(max_col, column);
        let pos = (column, row);
        result.insert(pos, match chr {
          '#' => Tile::Wall,
          _ => Tile::Floor,
        });
      }
    }

    (result, (max_row, max_col + 1))
  }

  fn load_units(input: &str) -> Vec<Unit> {
    let mut result = vec![];
    for (row, line) in input.split('\n').enumerate() {
      for (column, chr) in line.chars().enumerate() {
        let pos = (column, row);
        match chr {
          'E' => {
            result.push(Unit::new(pos, Kind::Elf));
          },
          'G' => {
            result.push(Unit::new(pos, Kind::Goblin));
          },
          _ => {}
        }
      }
    }

    result
  }
}

impl FromStr for Cave {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Cave, ParseError> {
    let (board, dimension) = Cave::load_board(s);
    let units = Cave::load_units(s);

    Ok(Cave { board, units, dimension, debug: false })
  }
}

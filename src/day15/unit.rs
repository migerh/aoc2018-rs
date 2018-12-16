use std::collections::{BTreeSet, BTreeMap};
use super::cave::{Board, Cave, Tile};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Kind {
  Elf,
  Goblin
}

pub type Position = (usize, usize);

fn position_add(pos: Position, x: i32, y: i32) -> Position {
  ((pos.0 as i32 + x) as usize, (pos.1 as i32 + y) as usize)
}

fn distance(a: Position, b: Position) -> usize {
  (b.1 as i32 - a.1 as i32).abs() as usize + (b.0 as i32 - a.0 as i32).abs() as usize
}

fn log(s: String, out: bool) {
  if out {
    println!("{}", s);
  }
}

#[derive(Debug, Clone)]
pub struct Unit {
  pub position: Position,
  pub kind: Kind,
  pub attack: i32,
  pub health: i32,
  pub tick: u64,
}

impl Unit {
  pub fn new(position: Position, kind: Kind) -> Self {
    let attack = 3;
    let health = 200;
    let tick = 0;

    Unit { position, kind, attack, health, tick }
  }

  fn reachable_positions(&self, board: &Board) -> BTreeMap<Position, u32> {
    Unit::reachable_positions_from(self.position, board)
  }

  fn reachable_positions_from(pos: Position, board: &Board) -> BTreeMap<Position, u32> {
    let mut result = BTreeMap::new();
    let mut last_count = 0;
    // let search = vec![-1, 0, 1];
    let mut round = 1;

    result.insert(pos, 0);
    while last_count != result.len() {
      last_count = result.len();
      let mut additional_positions = BTreeSet::new();
      for (pos, _) in &result {
        let range = Unit::in_range_of_position(*pos, board);
        for p in range {
          if board.get(&p) == Some(&Tile::Floor) {
            additional_positions.insert(p);
          }
        }
      }

      for pos in additional_positions {
        result.entry(pos).or_insert(round);
      }
      round += 1;
    }

    // result.insert(self.position);
    // while last_count != result.len() {
    //   last_count = result.len();
    //   let mut additional_positions = BTreeSet::new();
    //   for pos in &result {
    //     for dy in &search {
    //       for dx in &search {
    //         let check_pos = position_add(*pos, *dx, *dy);
    //         if board.get(&check_pos) == Some(&Tile::Floor) {
    //           additional_positions.insert(check_pos);
    //         }
    //       }
    //     }
    //   }

    //   for pos in additional_positions {
    //     result.insert(pos);
    //   }
    // }

    result
  }

  fn in_range_of_position(position: Position, board: &Board) -> Vec<Position> {
    let mut result = vec![];

    let pos = (position.0 - 1, position.1);
    if board.get(&pos) == Some(&Tile::Floor) {
      result.push(pos);
    }

    let pos = (position.0, position.1 - 1);
    if board.get(&pos) == Some(&Tile::Floor) {
      result.push(pos);
    }

    let pos = (position.0 + 1, position.1);
    if board.get(&pos) == Some(&Tile::Floor) {
      result.push(pos);
    }

    let pos = (position.0, position.1 + 1);
    if board.get(&pos) == Some(&Tile::Floor) {
      result.push(pos);
    }

    result
  }

  fn in_range(&self, board: &Board) -> Vec<Position> {
    Unit::in_range_of_position(self.position, board)
  }

  fn first_position(positions: &[Position]) -> Option<Position> {
    if positions.is_empty() {
      return None;
    }

    let mut first = (100, 100);
    for pos in positions {
      if pos.1 < first.1 || (pos.1 == first.1 && pos.0 <= first.0) {
        first = *pos;
      }
    }

    Some(first)
  }

  fn find_closest_positions(target_positions: &Vec<Position>, distance_map: &BTreeMap<Position, u32>) -> (Vec<Position>, u32) {
    let mut target_distance_map = BTreeMap::new();
    let mut closest_distance = std::u32::MAX;
    for pos in target_positions {
      if !distance_map.contains_key(&pos) {
        continue;
      }

      let distance = distance_map[&pos];
      target_distance_map
        .entry(distance)
        .and_modify(|v: &mut Vec<Position>| v.push(pos.clone()))
        .or_insert(vec![pos.clone()]);
      if distance < closest_distance {
        closest_distance = distance;
      }
    }

    if target_distance_map.is_empty() {
      return (vec![], std::u32::MAX);
    }

    (target_distance_map[&closest_distance].clone(), closest_distance)
  }

  pub fn move_unit(&self, cave: &Cave) -> Option<Position> {
    let enemies: Vec<Unit> = cave.units
      .iter()
      .cloned()
      .filter(|v| v.kind != self.kind)
      .collect();

    if enemies.is_empty() {
      return None;
    }

    // we already have a unit in range
    let board = cave.burn_units_in(self.position);
    for enemy in &enemies {
      let range = enemy.in_range(&board);
      if range.contains(&self.position) {
        return None;
      }
    }

    // find all possible positions near enemies to move to
    let reachable_positions = self.reachable_positions(&board);
    log(format!("Reachable positions: {:?}", reachable_positions), cave.debug);

    let mut target_positions = vec![];
    for enemy in &enemies {
      let all_target_positions = enemy.in_range(&board);
      for target in all_target_positions {
        if reachable_positions.contains_key(&target) {
          target_positions.push(target);
        }
      }
    }

    // find the closest ones
    let (target_distances, _) = Unit::find_closest_positions(&target_positions, &reachable_positions);
    log(format!("closest positions: {:?}", target_distances), cave.debug);
    let target = match Unit::first_position(&target_distances) {
      Some(v) => v,
      None => return None
    };

    // let mut target_distance_map = BTreeMap::new();
    // let mut closest_distance = 128;
    // for pos in target_positions {
    //   let distance = reachable_positions[&pos];
    //   target_distance_map
    //     .entry(distance)
    //     .and_modify(|v: &mut Vec<Position>| v.push(pos))
    //     .or_insert(vec![pos]);
    //   if distance < closest_distance {
    //     closest_distance = distance;
    //   }
    // }
    // println!("target position candidates: {:?}", target_distance_map);
    // println!("closest positions: {:?}", target_distance_map.get(&closest_distance));
    // let target = match target_distance_map.get(&closest_distance) {
    //   Some(v) => Unit::first_position(v),
    //   None => return
    // };

    log(format!("Target position: {:?}", target), cave.debug);

    // find the position to actually move to
    let my_range = self.in_range(&board);
    let reachable_positions_of_target = Unit::reachable_positions_from(target, &board);
    let (targets, minimum_distance) = Unit::find_closest_positions(&my_range, &reachable_positions_of_target);
    log(format!("Possible target positions: {:?}", targets), cave.debug);
    let move_to = Unit::first_position(&targets);
    // let mut move_to = (0, 0);
    // let mut minimum_distance = 1000;
    // for candidate in my_range {
    //   let move_is_possible = reachable_positions_of_target.contains_key(&candidate);
    //   let distance_candidate_to_target = reachable_positions_of_target[&candidate];
    //   let distance_is_smaller = distance_candidate_to_target < minimum_distance;
    //   if move_is_possible && distance_is_smaller {
    //     minimum_distance = distance_candidate_to_target;
    //     move_to = candidate;
    //   }
    // }

    log(format!("Final destination is {:?} with a distance of {} to the target position of {:?}", move_to, minimum_distance, target), cave.debug);
    move_to
  }

  pub fn attack(&self, all: &mut Vec<Unit>) {
    let enemies = all
      .iter()
      .filter(|v| v.kind != self.kind);

    // for enemy in enemies {

    // }
  }
}
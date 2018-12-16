use std::collections::{BTreeSet, BTreeMap};
use std::cmp::min;
use super::cave::{Board, Cave, Tile};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Kind {
  Elf,
  Goblin
}

pub type Position = (usize, usize);

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

    log(format!("Target position: {:?}", target), cave.debug);

    // find the position to actually move to
    let my_range = self.in_range(&board);
    let reachable_positions_of_target = Unit::reachable_positions_from(target, &board);
    let (targets, minimum_distance) = Unit::find_closest_positions(&my_range, &reachable_positions_of_target);
    log(format!("Possible target positions: {:?}", targets), cave.debug);
    let move_to = Unit::first_position(&targets);

    log(format!("Final destination is {:?} with a distance of {} to the target position of {:?}", move_to, minimum_distance, target), cave.debug);
    move_to
  }

  pub fn attack(&self, cave: &Cave) -> Option<(usize, i32)> {
    // find enemies in range
    let my_range = self.in_range(&cave.board);
    let enemies_in_range: Vec<(usize, &Unit)> = cave.units
      .iter()
      .enumerate()
      .filter(|(_, v)| v.kind != self.kind)
      .filter(|(_, v)| my_range.contains(&v.position))
      .collect();

    if enemies_in_range.is_empty() {
      return None;
    }

    // find enemies with the lowest number of health
    let mut lowest_health = std::i32::MAX;
    let mut enemy_health_map = BTreeMap::new();
    for entry in enemies_in_range {
      let (_, enemy) = entry;
      lowest_health = min(lowest_health, enemy.health);
      enemy_health_map
        .entry(enemy.health)
        .and_modify(|v: &mut Vec<(usize, &Unit)>| v.push(entry))
        .or_insert(vec![entry]);
    }

    if enemy_health_map.is_empty() {
      return None;
    }

    // find the position of the enemy to attack
    let attack_targets = &enemy_health_map[&lowest_health];
    let positions: Vec<Position> = attack_targets.iter().map(|(_, v)| v.position).collect();
    let position_to_attack = match Unit::first_position(&positions) {
      Some(v) => v,
      None => return None
    };

    // find the index of the enemy on that position
    let target: Vec<(usize, &Unit)> = attack_targets.iter().cloned().filter(|(_, v)| v.position == position_to_attack).collect();

    if target.len() > 1 {
      panic!("Two units in the same spot?!?");
    }

    if target.is_empty() {
      return None;
    }

    Some((target[0].0, self.attack))
  }
}
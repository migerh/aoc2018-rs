use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

type Position = (u64, u64);

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Hash)]
enum Tool {
  ClimbingGear,
  Torch,
  Neither,
}

static DEPTH: u64 = 10689;
static TARGET: Position = (11, 722);

fn geologic_index(p: Position, target: Position, lut: &HashMap<Position, u64>) -> u64 {
  if p.0 == 0 && p.1 == 0 {
    0
  } else if p.0 == target.0 && p.1 == target.1 {
    0
  } else if p.0 == 0 {
    p.1 * 48271
  } else if p.1 == 0 {
    p.0 * 16807
  } else {
    lut[&(p.0 - 1, p.1)] * lut[&(p.0, p.1 - 1)]
  }
}

fn erosion_level(p: Position, depth: u64, target: Position, lut: &mut HashMap<Position, u64>) -> u64 {
  let index = (geologic_index(p, target, lut) + depth) % 20183;
  lut.insert(p, index);
  index
}

fn risk_level(depth: u64, target: Position) -> u64 {
  let mut risk_level = 0;
  let mut lut = HashMap::new();
  for x in 0..target.0+1 {
    for y in 0..target.1+1 {
      risk_level += erosion_level((x, y), depth, target, &mut lut) % 3;
    }
  }

  risk_level
}

pub fn problem1() {
  let result = risk_level(DEPTH, TARGET);
  println!("Risk level: {}", result);
}

// type Seed = (u64, Position, Tool);

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Seed {
  pub time: u64,
  pub pos: Position,
  pub tool: Tool,
}

impl PartialOrd for Seed {
  fn partial_cmp(&self, other: &Seed) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl Ord for Seed {
  fn cmp(&self, other: &Seed) -> Ordering {
    other.time.cmp(&self.time)
      .then_with(|| other.pos.cmp(&self.pos))
  }
}

impl Seed {
  pub fn new(time: u64, pos: Position, tool: Tool) -> Seed {
    Seed { pos, tool, time }
  }
}

fn get_forbidden_tool_from_risk(risk: u64) -> Tool {
  match risk {
    0 => Tool::Neither,
    1 => Tool::Torch,
    2 => Tool::ClimbingGear,
    _ => panic!("This can't happen")
  }
}

fn build_map(depth: u64, max: Position, target: Position) -> HashMap<Position, Tool> {
  let mut lut = HashMap::new();
  let mut map = HashMap::new();
  for x in 0..max.0+1 {
    for y in 0..max.1+1 {
      let risk = erosion_level((x, y), depth, target, &mut lut) % 3;
      let forbidden_tool = get_forbidden_tool_from_risk(risk);
      map.insert((x, y), forbidden_tool);
    }
  }

  map
}

fn find_neighbours(p: Position, max: Position) -> Vec<Position> {
  let mut neighbours = vec![];
  if p.0 > 0 {
    neighbours.push((p.0 - 1, p.1));
  }

  if p.0 + 1 < max.0 {
    neighbours.push((p.0 + 1, p.1));
  }

  if p.1 > 0 {
    neighbours.push((p.0, p.1 - 1));
  }

  if p.1 + 1 < max.1 {
    neighbours.push((p.0, p.1 + 1));
  }

  neighbours
}

fn find_tool(forbidden_now: &Tool, forbidden_then: &Tool) -> Tool {
  match (forbidden_now, forbidden_then) {
    (Tool::ClimbingGear, Tool::Neither) => Tool::Torch,
    (Tool::Neither, Tool::ClimbingGear) => Tool::Torch,
    (Tool::ClimbingGear, Tool::Torch) => Tool::Neither,
    (Tool::Torch, Tool::ClimbingGear) => Tool::Neither,
    (Tool::Neither, Tool::Torch) => Tool::ClimbingGear,
    (Tool::Torch, Tool::Neither) => Tool::ClimbingGear,
    _ => panic!("This should not happen!!!")
    // (Tool::ClimbingGear, Tool::ClimbingGear) => Tool::Torch,
    // (Tool::Torch, Tool::Torch) => Tool::Neither,
    // (Tool::Neither, Tool::Neither) => Tool::ClimbingGear,
  }
}

fn manhattan(a: Position, b: Position) -> u64 {
  ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as u64
}

fn shortest_path(
  map: &HashMap<Position, Tool>,
  path_map: &mut HashMap<Position, (u64, Tool)>,
  seed: &Seed,
  seeds: &mut BinaryHeap<Seed>,
  max: Position
) {
  let neighbours = find_neighbours(seed.pos, max);
  let forbidden_now = &map[&seed.pos];

  for n in neighbours {
    let mut time = 1;
    let forbidden_then = &map[&n];
    let new_tool = if seed.tool == *forbidden_then {
      time = 8;
      find_tool(forbidden_now, forbidden_then)
    } else {
      seed.tool.clone()
    };

    let new_distance = seed.time + time;

    // if our path is already longer thant what we have
    // at target, abandon that path
    if let Some(distance_at_target) = path_map.get(&TARGET) {
      if distance_at_target.0 < new_distance {
        continue;
      }
    }

    if !path_map.contains_key(&n) {
      seeds.push(Seed::new(new_distance, n, new_tool.clone()));
    }

    path_map
      .entry(n)
      .and_modify(|v| {
        if new_distance <= v.0 {
          seeds.push(Seed::new(new_distance, n, new_tool.clone()));
          *v = (new_distance, new_tool.clone());
        }
      })
      .or_insert((new_distance, new_tool.clone()));
  }
}

fn purge_seeds(seeds: &mut BinaryHeap<Seed>, current: &Seed) -> BinaryHeap<Seed> {
  seeds.iter().cloned().filter(|seed| seed.pos != current.pos || (seed.pos == current.pos && seed.tool != current.tool)).collect()
}

pub fn problem2() {
  let max = (2000, 2000);
  let target = TARGET;
  let map = build_map(DEPTH, max, target);

  // let max = (35, 35);
  // let target = (10, 10);
  // let map = build_map(510, max, (10, 10));

  let seed = Seed::new(0, (0, 0), Tool::Torch);
  let mut path_map = HashMap::new();
  path_map.insert((0, 0), (0, Tool::Torch));

  let mut visited = HashSet::new();
  let mut seeds = BinaryHeap::new();
  seeds.push(seed);

  println!("Initial seeds: {:?}", seeds);
  let mut i = 0;
  while let Some(seed) = seeds.pop() {
    if visited.contains(&(seed.pos, seed.tool.clone())) {
      continue;
    }
    // println!("Number of seeds: {}", seeds.len());
    seeds = purge_seeds(&mut seeds, &seed);
    visited.insert((seed.pos, seed.tool.clone()));
    // if visited.contains(&target) {
    //   break;
    // }

    shortest_path(&map, &mut path_map, &seed, &mut seeds, max);

    i += 1;
    if i % 100_000 == 0 {
      println!("{}% covered", (100 * visited.len()) / (max.0 * max.1) as usize);
      println!("Currently have {} seeds", seeds.len());
      println!("Result? {:?}", path_map.get(&target));
    }
  }

  let result = path_map.get(&target);
  println!("Result: {:?} (after {} iterations)", result, i);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_example() {
    assert_eq!(risk_level(510, (10, 10)), 114);
  }
}
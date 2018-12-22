use std::collections::{HashMap, HashSet, BTreeSet};

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone)]
struct Seed {
  pub time: u64,
  pub pos: Position,
  pub tool: Tool,
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

fn find_neighbours(p: Position, max: Position, visited: &HashSet<Position>) -> Vec<Position> {
  let mut neighbours = vec![];
  if p.0 > 0 && !visited.contains(&(p.0 - 1, p.1)) {
    neighbours.push((p.0 - 1, p.1));
  }

  if p.0 + 1 < max.0 && !visited.contains(&(p.0 + 1, p.1)) {
    neighbours.push((p.0 + 1, p.1));
  }

  if p.1 > 0 && !visited.contains(&(p.0, p.1 - 1)) {
    neighbours.push((p.0, p.1 - 1));
  }

  if p.1 + 1 < max.1 && !visited.contains(&(p.0, p.1 + 1)) {
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
  visited: &HashSet<Position>,
  seed: Seed,
  max: Position
) -> BTreeSet<Seed> {
  let mut new_seeds = BTreeSet::new();
  let neighbours = find_neighbours(seed.pos, max, visited);
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
      if distance_at_target.0 < new_distance + manhattan(TARGET, n) - 2 {
        continue;
      }
    }

    if !path_map.contains_key(&n) {
      new_seeds.insert(Seed::new(new_distance, n, new_tool.clone()));
    }

    path_map
      .entry(n)
      .and_modify(|v| {
        if new_distance <= v.0 {
          new_seeds.insert(Seed::new(new_distance, n, new_tool.clone()));
          *v = (new_distance, new_tool.clone());
        }
      })
      .or_insert((new_distance, new_tool.clone()));
  }

  new_seeds
}

fn smallest_seed(seeds: &mut BTreeSet<Seed>) -> Option<Seed> {
  if seeds.is_empty() {
    return None;
  }

  let finding = seeds.iter().cloned().next();
  // println!("Smallest seed: {:?}", finding);

  if let Some(seed) = finding {
    seeds.remove(&seed);
    return Some(seed);
  }

  None
  // let mut smallest = 0;
  // let mut time = std::u64::MAX;

  // for (index, seed) in seeds.iter().enumerate() {
  //   if seed.time < time {
  //     time = seed.time;
  //     smallest = index;
  //   }
  // }

  // let seed = seeds.remove(smallest);
  // Some(seed)
}

pub fn problem2() {
  let max = (1100, 1100);
  let target = TARGET;
  let map = build_map(DEPTH, max, target);

  // let max = (20, 20);
  // let target = (10, 10);
  // let map = build_map(510, max, (10, 10));

  let seed = Seed::new(0, (0, 0), Tool::Torch);
  let mut path_map = HashMap::new();
  path_map.insert((0, 0), (0, Tool::Torch));

  let mut visited = HashSet::new();

  let mut seeds = shortest_path(&map, &mut path_map, &visited, seed, max);
  println!("Initial seeds: {:?}", seeds);
  let mut i = 0;
  while let Some(seed) = smallest_seed(&mut seeds) {
    // println!("Number of seeds: {}", seeds.len());
    visited.insert(seed.pos);
    // if visited.contains(&TARGET) {
    //   break;
    // }

    let mut new_seeds = shortest_path(&map, &mut path_map, &visited, seed, max);
    seeds.append(&mut new_seeds);

    // if let Some(target_time) = path_map.get(&TARGET) {
    //   break;
    // }

    i += 1;
    if i % 100_000 == 0 {
      println!("{}% covered", (100 * path_map.len()) / (max.0 * max.1) as usize);
      println!("Currently have {} seeds", seeds.len());
      println!("Result? {:?}", path_map.get(&TARGET));
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
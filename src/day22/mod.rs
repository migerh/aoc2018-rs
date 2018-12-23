use std::collections::{HashMap, HashSet, BTreeMap};

type Position = (u64, u64);

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Hash, Copy)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
struct Node {
  pub time: u64,
  pub pos: Position,
  pub tool: Tool,
}

impl Node {
  pub fn new(time: u64, pos: Position, tool: Tool) -> Node {
    Node { pos, tool, time }
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone)]
struct Visited {
  pub pos: Position,
  pub tool: Tool,
}

impl Visited {
  pub fn new(pos: Position, tool: Tool) -> Visited {
    Visited { pos, tool }
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

fn find_neighbours(p: Position) -> Vec<Position> {
  let mut neighbours = vec![];
  if p.0 > 0 {
    neighbours.push((p.0 - 1, p.1));
  }

  if p.1 > 0 {
    neighbours.push((p.0, p.1 - 1));
  }

  neighbours.push((p.0 + 1, p.1));
  neighbours.push((p.0, p.1 + 1));

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

fn prioritized_node(backlog: &mut BTreeMap<u64, Vec<Node>>) -> Option<Node> {
  if let Some(first_non_empty_log) = backlog.iter_mut().filter(|(_, v)| !v.is_empty()).next() {
    Some(first_non_empty_log.1.remove(0))
  } else {
    None
  }
}

pub fn problem2() {
  let max = (1000, 1000);
  let target = TARGET;
  let map = build_map(DEPTH, max, target);

  // let max = (20, 20);
  // let target = (10, 10);
  // let map = build_map(510, max, (10, 10));

  let mut visited: HashSet<Visited> = HashSet::new();
  let mut backlog: BTreeMap<u64, Vec<Node>> = BTreeMap::new();
  backlog.insert(0, vec![Node::new(0, (0, 0), Tool::Torch)]);

  let mut i = 0;
  while let Some(current) = prioritized_node(&mut backlog) {
    i += 1;
    if current.pos == target {
      // We are at the target position and have a torch in hand
      if current.tool == Tool::Torch {
        println!("Found target in {} minutes", current.time);
        break;
      // we are at the target pos but have to switch to a torch
      } else {
        let new_node = Node::new(current.time + 7, current.pos, Tool::Torch);
        println!("Final switch {:?}", new_node);
        backlog.entry(current.time + 7)
          .and_modify(|v| v.push(new_node))
          .or_insert(vec![new_node]);
      }
    }

    let v = Visited::new(current.pos, current.tool);
    if visited.contains(&v) {
      continue;
    }
    visited.insert(v);

    let neighbours = find_neighbours(current.pos);
    for n in neighbours {
      let new_node = if current.tool != map[&n] {
        Node::new(current.time + 1, n, current.tool)
      } else {
        Node::new(current.time + 7, current.pos, find_tool(&map[&current.pos], &map[&n]))
      };
      backlog.entry(new_node.time)
        .and_modify(|v| v.push(new_node))
        .or_insert(vec![new_node]);
    }
  }

  println!("Ran for {} iterations", i);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_example() {
    assert_eq!(risk_level(510, (10, 10)), 114);
  }
}
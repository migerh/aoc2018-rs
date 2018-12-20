use std::collections::HashSet;
use super::node::Node;

pub type Position = (i32, i32);

fn delta_pos(c: char) -> Position {
  match c {
    'E' => (-1, 0),
    'N' => (0, -1),
    'W' => (1, 0),
    'S' => (0, 1),
    _ => (0, 0),
  }
}

pub fn shortest_path(node: Box<Node>, pos: &mut Position, map: &mut HashSet<Position>, base_length: usize) -> usize {
  let mut length = base_length;
  for n in node.buffer.chars() {
    let delta = delta_pos(n);
    pos.0 += delta.0;
    pos.1 += delta.1;
    if !map.contains(pos) {
      map.insert(*pos);
      length += 1;
    }
  }

  if node.children.len() == 0 {
    return length;
  }

  let mut lengths = vec![];
  for child in node.children {
    lengths.push(shortest_path(child, pos, map, length));
  }

  let result = match lengths.iter().max() {
    Some(v) => v,
    None => panic!("This should not happen")
  };

  *result
}
use std::collections::HashSet;
use super::node::Directions;

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

fn walk_and_count_new_rooms(directions: &String, pos: &mut Position, map: &mut HashSet<Position>) -> usize {
  let mut length = 0;
  for n in directions.chars() {
    let delta = delta_pos(n);
    pos.0 += delta.0;
    pos.1 += delta.1;
    if !map.contains(pos) {
      map.insert(*pos);
      length += 1;
    } else {
      length = 0;
    }
  }

  length
}

pub fn count_rooms(direction: &Directions, mut pos: &mut Position, mut map: &mut HashSet<Position>, indent: usize) -> usize {
  let mut spaces = "".to_string();
  let space = ' ';
  for _i in 0..indent {
    spaces.push(space);
  }

  match direction {
    Directions::Content(v) => {
      let result = walk_and_count_new_rooms(&v, &mut pos, &mut map);
      println!("{}-- Content: {} -> {}", spaces, v, result);
      result
    },
    Directions::Options(v) => {
      println!("{}Options: {:?}", spaces, v);
      let mut lengths = vec![];
      let old_pos = pos.clone();
      for d in v {
        lengths.push(count_rooms(d, &mut pos, &mut map, indent + 1));
        pos.0 = old_pos.0;
        pos.1 = old_pos.1;
      }
      if let Some(m) = lengths.iter().max() {
        println!("{}-- Options: {}", spaces, m);
        *m
      } else {
        println!("{}No options found", spaces);
        0
      }
    },
    Directions::Concat(v) => {
      println!("{}Concat: {:?}", spaces, v);
      let result = v.iter().map(|v| count_rooms(v, &mut pos, &mut map, indent + 1)).sum();
      println!("{}-- Concat: {}", spaces, result);
      result
    }
  }
}
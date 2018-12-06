use std::collections::BTreeMap;
use std::option::Option;

use super::utils::{preprocess_input, ParseError};

fn parse_coordinate(s: &str) -> Result<(i32, i32), ParseError> {
  let mut split = s.split(",");

  let x_str = match split.next() {
    Some(v) => v,
    None => Err(ParseError::new("Could not parse coordinate"))?
  };

  let x = x_str.trim().parse::<i32>()?;

  let y_str = match split.next() {
    Some(v) => v,
    None => Err(ParseError::new("Could not parse coordinate"))?
  };
  let y = y_str.trim().parse::<i32>()?;

  Ok((x, y))
}

fn manhattan_distance(p: (i32, i32), q: (i32, i32)) -> i32 {
  (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn find_closest_point(q: (i32, i32), points: &Vec<(i32, i32)>) -> (Option<(i32, i32)>, i32) {
  let mut closest_point = Some((0, 0));
  let mut closest_distance = 1000000;
  for p in points {
    let distance = manhattan_distance(q, *p);
    if distance == closest_distance {
      closest_point = None;
    }
    if distance < closest_distance {
      closest_distance = distance;
      closest_point = Some(*p);
    }
  }

  (closest_point, closest_distance)
}

fn calculate_map(coords: &Vec<(i32, i32)>, start: (i32, i32), end: (i32, i32)) -> BTreeMap<(i32, i32), (Option<(i32, i32)>, i32)> {
  let mut map = BTreeMap::new();
  for x in start.0..end.0 {
    for y in start.1..end.1 {
      let closest_point_and_distance = find_closest_point((x, y), &coords);
      map.insert((x, y), closest_point_and_distance);
    }
  }
  map
}

fn is_infinite(p: (i32, i32)) -> bool {
  p.0 == 0 || p.1 == 0 || p.0 == 499 || p.1 == 499
}

pub fn problem1() -> Result<i32, ParseError> {
  let input = include_str!("./data/input.txt");

  let coords = preprocess_input(input)
    .into_iter()
    .map(|v| parse_coordinate(v))
    .collect::<Result<Vec<(i32, i32)>, _>>()?;

  let map = calculate_map(&coords, (0, 0), (500, 500));

  let mut area_map = BTreeMap::new();
  for (point, (closest_point, _distance)) in &map {
    match closest_point {
      Some(p) => {
        area_map.entry(p).and_modify(|v| *v += 1).or_insert(1);
        if is_infinite(*point) {
          area_map.entry(p).and_modify(|v| *v = -50000);
        }
      },
      None => ()
    }
  }

  let mut largest_area = 0;
  for (_point, area) in area_map {
    if largest_area < area {
      largest_area = area;
    }
  }

  println!("largest area: {}", largest_area);

  Ok(largest_area)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn manhatten_distance_works() {
    let p = (0, 0);
    let q = (6, 6);
    assert_eq!(manhattan_distance(p, q), 12);
  }

  #[test]
  fn problem1_result_is_correct() {
    assert_eq!(problem1().unwrap(), 4398);
  }
}
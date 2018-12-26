use super::utils::{ParseError, Error};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
  pub x: i64,
  pub y: i64,
  pub z: i64,
  pub w: i64,
}

impl FromStr for Point {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Point, ParseError> {
    let coords = s
      .split(',')
      .map(|v| v.trim())
      .map(|v| v.parse::<i64>())
      .collect::<Result<Vec<i64>, ParseIntError>>()?;

    if coords.len() != 4 {
      Err(ParseError::new("Insufficient number of coordinates found"))
    } else {
      Ok(Point { x: coords[0], y: coords[1], z: coords[2], w: coords[3] })
    }
  }
}

impl Point {
  pub fn manhattan(p: &Point, q: &Point) -> i64 {
    (p.x - q.x).abs() +
    (p.y - q.y).abs() +
    (p.z - q.z).abs() +
    (p.w - q.w).abs()
  }
}

fn read_points() -> Result<Vec<Point>, ParseError> {
  let input = include_str!("./data/input.txt");

  input.split("\n")
    .filter(|v| *v != "")
    .map(|v| Point::from_str(v))
    .collect::<Result<Vec<Point>, ParseError>>()
}

fn find_next_close_point(points: &mut Vec<Point>, cluster: &mut Vec<Point>) -> bool {
  let mut to_remove = None;
  'outer: for (index, point) in points.iter().enumerate() {
    for p in cluster.iter() {
      if Point::manhattan(p, point) <= 3 {
        to_remove = Some(index);
        break 'outer;
      }
    }
  }

  if let Some(index) = to_remove {
    let p = points.remove(index);
    cluster.push(p);
    true
  } else {
    false
  }
}

fn cluster(points: &mut Vec<Point>) -> Vec<Vec<Point>> {
  let mut clusters = vec![];
  let mut cluster = vec![];

  while !points.is_empty() {
    let last = points.pop().unwrap();
    cluster.push(last);

    while find_next_close_point(points, &mut cluster) {
    }
    clusters.push(cluster);
    cluster = vec![];
  }

  clusters
}

fn print(points: &Vec<Point>) {
  for p in points {
    println!("{:?}", p);
  }
}

pub fn problem1() -> Result<(), Error> {
  let mut points = read_points()?;

  let clusters = cluster(&mut points);

  print(&points);
  println!("Number of clusters: {}", clusters.len());

  Ok(())
}
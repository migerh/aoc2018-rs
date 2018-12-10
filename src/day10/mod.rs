use std::str::FromStr;
use regex::Regex;
use super::utils::{preprocess_input, ParseError};

#[derive(Debug)]
struct Light {
  pub position: (i32, i32),
  pub velocity: (i32, i32),
}

impl FromStr for Light {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Light, ParseError> {
    lazy_static!{
      static ref RE: Regex = Regex::new(r"position=<\s*(-?\d+)\s*,\s*(-?\d+)\s*>\s*velocity=<\s*(-?\d+)\s*,\s*(-?\d+)\s*>").unwrap();
    }
    let cap = RE.captures(s).unwrap();

    let parse = |v: &str| v.parse::<i32>();

    let x = parse(&cap[1])?;
    let y = parse(&cap[2])?;
    let vx = parse(&cap[3])?;
    let vy = parse(&cap[4])?;

    Ok(Light { position: (x, y), velocity: (vx, vy) })
  }
}

fn forward(lights: &mut Vec<Light>) {
  for light in lights {
    light.position.0 += light.velocity.0;
    light.position.1 += light.velocity.1;
  }
}

fn backward(lights: &mut Vec<Light>) {
  for light in lights {
    light.position.0 -= light.velocity.0;
    light.position.1 -= light.velocity.1;
  }
}

fn bounding_box(lights: &Vec<Light>) -> ((i32, i32), (i32, i32)) {
  let mut max = (0, 0);
  let mut min = (100000, 100000);
  for l in lights {
    if l.position.0 > max.0 {
      max.0 = l.position.0;
    }

    if l.position.0 < min.0 {
      min.0 = l.position.0;
    }

    if l.position.1 > max.1 {
      max.1 = l.position.1;
    }

    if l.position.1 < min.1 {
      min.1 = l.position.1;
    }
  }

  (min, max)
}

fn print_lights(lights: &Vec<Light>, size: (i32, i32), min: (i32, i32)) {
  let mut display: Vec<Vec<char>> = vec![];

  for _i in 0..(size.1 + 1) {
    let line = vec![' '; (size.0 + 1) as usize];
    display.push(line);
  }

  for l in lights {
    let pos = l.position;
    let rx = pos.0 - min.0;
    let ry = pos.1 - min.1;
    display[ry as usize][rx as usize] = 'x';
  }

  for line in display {
    for c in line {
      print!("{}", c);
    }
    println!("");
  }
}

fn bounding_box_size(lights: &Vec<Light>) -> (i32, i32) {
  let (min, max) = bounding_box(lights);
  (max.0 - min.0, max.1 - min.1)
}

pub fn problem1() -> Result<(), ParseError> {
  let mut lights = preprocess_input(include_str!("./data/input.txt"))
    .iter()
    .cloned()
    .map(|s| Light::from_str(s))
    .collect::<Result<Vec<Light>, ParseError>>()?;

  let mut previous_bb_size = (1000000, 1000000);
  for _i in 0..100000 {
    forward(&mut lights);
    let bb_size = bounding_box_size(&lights);

    if previous_bb_size.0 < bb_size.0 || previous_bb_size.1 < bb_size.1 {
      backward(&mut lights);

      let bb = bounding_box(&lights);
      print_lights(&lights, previous_bb_size, bb.0);
      break;
    }
    previous_bb_size = bb_size;
  }

  Ok(())
}
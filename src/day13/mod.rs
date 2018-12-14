use std::collections::BTreeMap;
use super::utils::{preprocess_input, Error};

type Position = (i32, i32);

#[derive(Debug, Clone)]
enum Turn {
  Left,
  Straight,
  Right,
}

#[derive(Debug, Clone)]
struct Cart {
  pub position: Position,
  pub direction: Position,
  pub next_turn: Turn,
  pub broken: bool,
  pub tick: i32,
}

impl Cart {
  pub fn new(p: Position, init: char) -> Cart {
    let direction = match init {
      '>' => (1, 0),
      '<' => (-1, 0),
      'v' => (0, 1),
      '^' => (0, -1),
      _ => (0, 0),
    };
    let broken = false;
    Cart { position: p, direction, next_turn: Turn::Left, broken, tick: -1 }
  }

  pub fn from_cart(position: Position, direction: Position, next_turn: Turn, broken: bool, tick: i32) -> Cart {
    Cart { position, direction, next_turn, broken, tick }
  }
}

fn load_tracks() -> Vec<Vec<char>> {
  let input = include_str!("./data/input.txt");

  preprocess_input(input)
    .iter()
    .map(|v| v.chars().collect())
    .collect()
}

fn new_pos(p: Position, d: Position) -> Position {
  (p.0 + d.0, p.1 + d.1)
}

fn drive(cart: Cart, tracks: &Vec<Vec<char>>) -> Cart {
  let pos = new_pos(cart.position, cart.direction);

  let next = tracks[pos.1 as usize][pos.0 as usize];
  let (direction, next_turn) = match (next, cart.direction, cart.next_turn) {
    ('/', (0, 1), t) => ((-1, 0), t),
    ('/', (0, -1), t) => ((1, 0), t),
    ('/', (-1, 0), t) => ((0, 1), t),
    ('/', (1, 0), t) => ((0, -1), t),
    ('\\', (-1, 0), t) => ((0, -1), t),
    ('\\', (0, 1), t) => ((1, 0), t),
    ('\\', (1, 0), t) => ((0, 1), t),
    ('\\', (0, -1), t) => ((-1, 0), t),
    ('+', (0, 1), Turn::Left) => ((1, 0), Turn::Straight),
    ('+', (0, 1), Turn::Right) => ((-1, 0), Turn::Left),
    ('+', (0, -1), Turn::Left) => ((-1, 0), Turn::Straight),
    ('+', (0, -1), Turn::Right) => ((1, 0), Turn::Left),
    ('+', (1, 0), Turn::Left) => ((0, -1), Turn::Straight),
    ('+', (1, 0), Turn::Right) => ((0, 1), Turn::Left),
    ('+', (-1, 0), Turn::Left) => ((0, 1), Turn::Straight),
    ('+', (-1, 0), Turn::Right) => ((0, -1), Turn::Left),
    ('+', d, Turn::Straight) => (d, Turn::Right),
    (_, d, t) => (d, t)
  };

  Cart::from_cart(pos, direction, next_turn, cart.broken, cart.tick)
}

fn find_carts(tracks: &Vec<Vec<char>>) -> Vec<Cart> {
  let mut carts = vec![];

  for (y, line) in tracks.iter().enumerate() {
    for (x, &c) in line.iter().enumerate() {
      if c == 'v' || c == '>' || c == '^' || c == '<' {
        carts.push(Cart::new((x as i32, y as i32), c));
      }
    }
  }

  carts
}

fn find_collision(carts: &Vec<Cart>) -> Vec<usize> {
  let mut map = BTreeMap::new();

  for (index, cart) in carts.iter().enumerate() {
    if cart.broken {
      continue;
    }

    map
      .entry(cart.position)
      .and_modify(|v: &mut Vec<usize> | v.push(index))
      .or_insert(vec![index]);
  }

  let result = vec![];
  for (_key, value) in map {
    if value.len() > 1 {
      return value;
    }
  }

  result
}

pub fn problem1() -> Result<Position, Error> {
  let tracks = load_tracks();
  let mut carts = find_carts(&tracks);

  for i in 0..200 {
    let carts_copy: Vec<Cart> = carts
      .iter()
      .map(|v| v.clone())
      .collect();

    for y in 0..150 {
      for x in 0..150 {
        for (idx, c) in carts_copy.iter().enumerate() {
          let (cx, cy) = c.position;
          if cx == x && cy == y && c.tick < i {
            carts[idx] = drive(c.clone(), &tracks);
            carts[idx].tick = i;

            let collided = find_collision(&carts);
            if collided.len() > 0 {
              let position = carts[collided[0] as usize].position;
              println!("First collision detected at: {:?}", position);
              return Ok(position)
            }
          }
        }
      }
    }
  }

  Err(Error::new("No collision occurred"))
}

fn print_carts(carts: &Vec<Cart>) {
  for cart in carts {
    println!("{:?}", cart);
  }
}

pub fn problem2() -> Result<Position, Error> {
  let tracks = load_tracks();
  let mut carts = find_carts(&tracks);
  let mut last_cart: Position = (0, 0);

  for i in 0..15000 {
    for y in 0..150 {
      for x in 0..150 {
        let num_carts = carts.len();
        for idx in 0..num_carts {
          let c = carts[idx].clone();
          let (cx, cy) = c.position;
          if cx == x && cy == y && !c.broken && c.tick < i {
            carts[idx] = drive(c, &tracks);
            carts[idx].tick = i;

            let collided = find_collision(&carts);
            for q in collided {
              carts[q].broken = true;
            }
          }
        }
      }
    }

    let unbroken_carts = carts
      .iter()
      .cloned()
      .filter(|v| !v.broken)
      .collect::<Vec<Cart>>();

    if unbroken_carts.len() == 1 {
      println!("Only one cart left!");
      print_carts(&unbroken_carts);
      last_cart = unbroken_carts[0].position;
      break;
    }

    if unbroken_carts.len() == 0 {
      println!("No carts left!");
      break;
    }
  }

  Ok(last_cart)
}

#[cfg(not(debug_assertions))]
#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(not(debug_assertions))]
  #[test]
  fn check_problem1() {
    assert_eq!(problem1().unwrap(), (64, 57));
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn check_problem2() {
    assert_eq!(problem2().unwrap(), (136, 8));
  }
}
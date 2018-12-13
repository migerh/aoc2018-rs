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
  let input = include_str!("./data/input.txt");
  let tracks: Vec<Vec<char>> = preprocess_input(input)
    .iter()
    .map(|v| v.chars().collect())
    .collect();

  let mut carts = find_carts(&tracks);
  let mut collision_occurred = false;

  for c in &carts {
    println!("{:?}", c);
  }

  let mut collision: Position = (0, 0);
  for i in 0..200 {
    if i % 100 == 0 {
      println!("Iteration {}", i);
    }

    let carts_copy: Vec<Cart> = carts
      .iter()
      .map(|v| v.clone())
      .collect();

    for y in 0..150 {
      for x in 0..150 {
        for (idx, c) in carts_copy.iter().enumerate() {
          let (cx, cy) = c.position;
          if cx == x && cy == y {
            carts[idx] = drive(c.clone(), &tracks);

            let collided = find_collision(&carts);
            if collided.len() > 0 {
              println!("Collision detected! {:?}", collided);
              collision_occurred = true;
              collision = carts[collided[0] as usize].position;
              break;
            }
          }
        }
      }

      if collision_occurred {
        break;
      }
    }

    if collision_occurred {
      break;
    }
  }

  Ok(collision)
}

fn print_carts(carts: &Vec<Cart>) {
  for cart in carts {
    println!("{:?}", cart);
  }
}

pub fn problem2() -> Result<Position, Error> {
  let input = include_str!("./data/input.txt");
  let tracks: Vec<Vec<char>> = preprocess_input(input)
    .iter()
    .map(|v| v.chars().collect())
    .collect();

  let mut carts = find_carts(&tracks);

  for c in &carts {
    println!("{:?}", c);
  }

  let mut last_cart: Position = (0, 0);

  for i in 0..15000 {
    if i % 100 == 0 {
      println!("Iteration {}", i);
    }

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
              let n = carts[q].clone();
              println!("Mark cart {} as broken: {:?}", q, n);
              println!("deleting cart at {} {}", n.position.0, n.position.1);
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
use std::collections::{BTreeMap, BTreeSet};
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
    Cart { position: p, direction, next_turn: Turn::Left, broken }
  }

  pub fn from_cart(position: Position, direction: Position, next_turn: Turn) -> Cart {
    let broken = false;
    Cart { position, direction, next_turn, broken }
  }
}

fn new_pos(p: Position, d: Position) -> Position {
  (p.0 + d.0, p.1 + d.1)
}

fn drive(cart: Cart, tracks: &Vec<Vec<char>>) -> Cart {
  let (x, y) = cart.position;
  let current = tracks[y as usize][x as usize];

  let (direction, next_turn) = match (current, cart.direction, cart.next_turn) {
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

  let pos = new_pos(cart.position, direction);
  Cart::from_cart(pos, direction, next_turn)
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
      // println!("Collided carts: {:?}", value);
      // for v in &value {
      //   println!("Cart collided: {:?}", carts[*v].clone());
      // }
      return value;
    }
  }

  result
}

pub fn problem1() -> Result<(), Error> {
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

  Ok(())
}

fn print_carts(carts: &Vec<Cart>) {
  for cart in carts {
    println!("{:?}", cart);
  }
}

pub fn problem2() -> Result<(), Error> {
  let input = include_str!("./data/input.txt");
  let tracks: Vec<Vec<char>> = preprocess_input(input)
    .iter()
    .map(|v| v.chars().collect())
    .collect();

  let mut carts = find_carts(&tracks);

  for c in &carts {
    println!("{:?}", c);
  }

  for i in 0..10000 {
    if i % 100 == 0 {
      println!("Iteration {}", i);
    }

    for y in 0..150 {
      for x in 0..150 {
        let num_carts = carts.len();
        for idx in 0..num_carts {
          let c = carts[idx].clone();
          let (cx, cy) = c.position;
          if cx == x && cy == y && !c.broken {
            carts[idx] = drive(c, &tracks);

            let collided = find_collision(&carts);
            for q in collided {
              println!("Mark cart {} as broken", q);
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
      break;
    }

    if unbroken_carts.len() == 0 {
      println!("No carts left!");
      break;
    }
  }

  Ok(())
}
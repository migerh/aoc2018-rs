use std::collections::BTreeMap;

fn initialize() -> Vec<char> {
  let initial_state = ".#####.##.#.##...#.#.###..#.#..#..#.....#..####.#.##.#######..#...##.#..#.#######...#.#.#..##..#.#.#";

  let plants = initial_state
    .chars()
    .collect();

  plants
}

fn process(current: Vec<char>, rules: &BTreeMap<&'static str, char>) -> Vec<char> {
  let mut new_state = vec!['.'; current.len()];

  for i in 2..(current.len() - 2) {
    let extract: String = current[(i-2)..(i+3)].iter().collect();
    for (key, value) in rules {
      if extract == key.to_string() {
        new_state[i] = *value;
        break;
      }
    }
  }

  new_state
}

fn checksum(v: &Vec<char>) -> usize {
  let mut sum = 0;

  for (i, c) in v.iter().enumerate() {
    sum += match c {
      '#' => i - 500,
      _ => 0
    };
  }

  sum
}

pub fn iterate(iterations: usize) -> (usize, usize) {
  let mut state = vec!['.'; 500];
  let mut initial_state = initialize();
  state.append(&mut initial_state);
  state.append(&mut vec!['.'; 1500]);
  let rules = get_rules();
  let mut previous_checksum = 0;
  let mut diff = 0;

  for i in 0..iterations {
    state = process(state, &rules);

    if (i + 1) % 100 == 0 {
      let checksum = checksum(&state);
      diff = checksum - previous_checksum;
      previous_checksum = checksum;
    }
  }

  let result = checksum(&state);
  (result, diff)
}

pub fn problem1() -> usize {
  let result = iterate(20).0;

  println!("Result: {}", result);
  result
}

pub fn problem2() -> usize {
  let iterations = 300;
  let (r, d) = iterate(iterations);

  let result = (50_000_000_000 - 300) / 100 * d + r;

  println!("Result: {}", result);

  result
}


fn get_rules() -> BTreeMap<&'static str, char> {
  let mut map = BTreeMap::new();

  // #..#. => .
  map.insert("#..#.", '.');

  // ##... => #
  map.insert("##...", '#');

  // #.... => .
  map.insert("#....", '.');

  // #...# => #
  map.insert("#...#", '#');

  // ...#. => .
  map.insert("...#.", '.');

  // .#..# => #
  map.insert(".#..#", '#');

  // #.#.# => .
  map.insert("#.#.#", '.');

  // ..... => .
  map.insert(".....", '.');

  // ##.## => #
  map.insert("##.##", '#');

  // ##.#. => #
  map.insert("##.#.", '#');

  // ###.. => #
  map.insert("###..", '#');

  // #.##. => .
  map.insert("#.##.", '.');

  // #.#.. => #
  map.insert("#.#..", '#');

  // ##..# => #
  map.insert("##..#", '#');

  // ..#.# => #
  map.insert("..#.#", '#');

  // ..#.. => .
  map.insert("..#..", '.');

  // .##.. => .
  map.insert(".##..", '.');

  // ...## => #
  map.insert("...##", '#');

  // ....# => .
  map.insert("....#", '.');

  // #.### => #
  map.insert("#.###", '#');

  // #..## => #
  map.insert("#..##", '#');

  // ..### => #
  map.insert("..###", '#');

  // ####. => #
  map.insert("####.", '#');

  // .#.#. => #
  map.insert(".#.#.", '#');

  // .#### => .
  map.insert(".####", '.');

  // ###.# => #
  map.insert("###.#", '#');

  // ##### => #
  map.insert("#####", '#');

  // .#.## => .
  map.insert(".#.##", '.');

  // .##.# => .
  map.insert(".##.#", '.');

  // .###. => .
  map.insert(".###.", '.');

  // ..##. => .
  map.insert("..##.", '.');

  // .#... => #
  map.insert(".#...", '#');

  map
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_problem1() {
    assert_eq!(problem1(), 3725);
  }

  #[test]
  fn check_problem2() {
    assert_eq!(problem2(), 3100000000293);
  }
}
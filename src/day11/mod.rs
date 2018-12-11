fn cell_power_level(cell: (u16, u16), serial: i32) -> i32 {
  let (x, y) = cell;
  let rack_id = (x + 10) as i32;

  let mut powerlevel = rack_id;
  powerlevel *= y as i32;
  powerlevel += serial;
  powerlevel *= rack_id;

  let above_hundreds = (powerlevel / 1000) * 10;
  let below_hundreds = powerlevel / 100;
  powerlevel = below_hundreds - above_hundreds;

  powerlevel - 5
}

fn power_level_square(grid: &Vec<Vec<i32>>, top_left: (u16, u16), size: u16) -> i32 {
  let mut powerlevel = 0;
  let (tx, ty) = top_left;
  for x in 0..size {
    for y in 0..size {
      powerlevel += grid[(tx + x - 1) as usize][(ty + y - 1) as usize];
    }
  }

  powerlevel
}

fn powerlevel_grid(serial: i32) -> Vec<Vec<i32>> {
  let mut result = vec![];
  for x in 1..301 {
    let mut line = vec![];
    for y in 1..301 {
      line.push(cell_power_level((x, y), serial));
    }
    result.push(line);
  }

  result
}

pub fn problem1() -> ((u16, u16), i32) {
  let mut max_tl = (1, 1);
  let mut max_pl = 0;
  let size = 3;

  let levels_grid = powerlevel_grid(5177);

  for x in 1..299 {
    for y in 1..299 {
      let powerlevel = power_level_square(&levels_grid, (x, y), size);
      if max_pl < powerlevel {
        max_tl = (x, y);
        max_pl = powerlevel;
      }
    }
  }


  println!("Max powerlevel of {} at {:?}", max_pl, max_tl);

  (max_tl, max_pl)
}

pub fn problem2() -> ((u16, u16), u16, i32) {
  let mut max_tl = (1, 1);
  let mut max_pl = 0;
  let mut max_size = 1;

  let levels_grid = powerlevel_grid(5177);

  for size in 1..300 {
    println!("size: {}", size);
    let end = 300 + 1 - size;
    for x in 1..end {
      for y in 1..end {
        let powerlevel = power_level_square(&levels_grid, (x, y), size);
        if max_pl < powerlevel {
          max_tl = (x, y);
          max_pl = powerlevel;
          max_size = size;
        }
      }
    }
  }


  println!("Max powerlevel of {} at {:?} with size {}", max_pl, max_tl, max_size);

  (max_tl, max_size, max_pl)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cell_power_level_1() {
    assert_eq!(cell_power_level((3, 5), 8), 4);
  }

  #[test]
  fn cell_power_level_2() {
    assert_eq!(cell_power_level((122, 79), 57), -5);
  }

  #[test]
  fn cell_power_level_3() {
    assert_eq!(cell_power_level((217, 196), 39), 0);
  }

  #[test]
  fn cell_power_level_4() {
    assert_eq!(cell_power_level((101, 153), 71), 4);
  }
}
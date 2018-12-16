use std::str::FromStr;
use super::utils::Error;

mod cave;
mod unit;

pub fn problem1() -> Result<i32, Error> {
  let input = include_str!("./data/input.txt");
  let mut cave = cave::Cave::from_str(input)?;

  for i in 0..1000 {
    if i % 10 == 0 {
      println!("After {} rounds:", i);
      cave.print_with_units();
    }

    cave.tick();

    let number_of_goblins = cave.units.iter().filter(|v| v.kind == unit::Kind::Goblin).count();
    let number_of_elfs = cave.units.iter().filter(|v| v.kind == unit::Kind::Elf).count();

    if number_of_elfs == 0 || number_of_goblins == 0 {
      let remaining_health: i32 = cave.units.iter().map(|v| v.health).sum();
      let result = remaining_health * i;
      println!("After the final round:");
      cave.print_with_units();
      println!("Finished! Result is {} * {} = {}", remaining_health, i, result);
      return Ok(result);
    }
  }

  Ok(0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn position_equals_two_identical_positions_are_equal() {
    let a: unit::Position = (3, 5);
    let b: unit::Position = (3, 5);
    assert_eq!(a, b);
  }

  #[test]
  fn position_equals_two_different_positions_are_not_equal() {
    let a: unit::Position = (3, 5);
    let b: unit::Position = (3, 6);
    assert_ne!(a, b);
  }
}
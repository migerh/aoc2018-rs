use std::str::FromStr;
use super::utils::Error;

mod cave;
mod unit;

pub fn problem1() -> Result<(), Error> {
  let input = include_str!("./data/example3.txt");
  let mut cave = cave::Cave::from_str(input)?;

  cave.print_with_units();

  cave.move_units();
  cave.print_with_units();

  cave.move_units();
  cave.print_with_units();

  cave.move_units();
  cave.print_with_units();

  Ok(())
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
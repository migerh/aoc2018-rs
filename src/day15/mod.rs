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

pub fn problem2() -> Result<i32, Error> {
  let input = include_str!("./data/input.txt");
  let mut number_of_surviving_elves = 0;
  let mut remaining_health = 0;
  let mut result = 0;
  let mut rounds = 0;

  for attack_power in 4..50 {
    let mut cave = cave::Cave::from_str(input)?;
    let mut number_of_elves = 0;

    println!("Simulating with attack power {} for the elves", attack_power);
    for (index, unit) in cave.units.clone().iter().enumerate() {
      if unit.kind == unit::Kind::Elf {
        number_of_elves += 1;
        cave.units[index].attack = attack_power;
      }
    }

    for i in 0..1000 {
      cave.tick();

      let number_of_goblins = cave.units.iter().filter(|v| v.kind == unit::Kind::Goblin).count();
      number_of_surviving_elves = cave.units.iter().filter(|v| v.kind == unit::Kind::Elf).count();

      if number_of_surviving_elves == 0 || number_of_goblins == 0 {
        remaining_health = cave.units.iter().map(|v| v.health).sum();
        result = remaining_health * i;
        rounds = i;
        break;
      }
    }

    println!("Simulation finished! Result is {} * {} = {}", remaining_health, rounds, result);
    println!("Number of elves died: {}", number_of_elves - number_of_surviving_elves);

    if number_of_surviving_elves == number_of_elves {
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
use std::collections::VecDeque;
use super::utils::Error;

trait Rotation<T> {
  fn rotate_forward(self: &mut Self, number: usize) -> Result<(), Error>;
  fn rotate_backward(self: &mut Self, number: usize) -> Result<(), Error>;
}

impl<T> Rotation<T> for VecDeque<T> {
  fn rotate_forward(self: &mut VecDeque<T>, number: usize) -> Result<(), Error> {
    for _i in 0..number {
      let front = match self.pop_front() {
        Some(v) => v,
        None => Err(Error::new("Could not rotate forward"))?
      };
      self.push_back(front);
    }
    Ok(())
  }

  fn rotate_backward(self: &mut VecDeque<T>, number: usize) -> Result<(), Error> {
    for _i in 0..number {
      let front = match self.pop_back() {
        Some(v) => v,
        None => Err(Error::new("Could not rotate backward"))?
      };
      self.push_front(front);
    }
    Ok(())
  }
}

fn play(number_of_players: u64, last_marble: u64) -> Result<u64, Error> {
  let mut circle = VecDeque::with_capacity((2 * last_marble) as usize);
  circle.push_back(0);
  let mut player = 0;
  let mut scores = vec![0; number_of_players as usize];

  for marble in 1..(last_marble+1) {
    player = (player % number_of_players) + 1;

    if marble % 23 == 0 {
      circle.rotate_backward(7)?;
      let score = match circle.pop_front() {
        Some(v) => v,
        None => Err(Error::new("Something went wrong"))?
      };
      scores[(player - 1) as usize] += score + marble;

      continue;
    }

    circle.rotate_forward(2)?;
    circle.push_front(marble);
  }

  Ok(scores.iter().cloned().fold(0, u64::max))
}

pub fn problem1() -> Result<u64, Error> {
  let players = 405;
  let last_marble = 71700;
  let highscore = play(players, last_marble)?;

  println!("With {} players and last marble {}, highscore is {}", players, last_marble, highscore);
  Ok(highscore)
}

pub fn problem2() -> Result<u64, Error> {
  let players = 405;
  let last_marble = 7170000;
  let highscore = play(players, last_marble)?;

  println!("With {} players and last marble {}, highscore is {}", players, last_marble, highscore);

  Ok(highscore)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(play(9, 25).unwrap(), 32);
  }

  #[test]
  fn example2() {
    assert_eq!(play(10, 1618).unwrap(), 8317);
  }

  #[test]
  fn example3() {
    assert_eq!(play(13, 7999).unwrap(), 146373);
  }

  #[test]
  fn example4() {
    assert_eq!(play(17, 1104).unwrap(), 2764);
  }

  #[test]
  fn example5() {
    assert_eq!(play(21, 6111).unwrap(), 54718);
  }

  #[test]
  fn example6() {
    assert_eq!(play(30, 5807).unwrap(), 37305);
  }

  #[test]
  fn check_problem1() {
    assert_eq!(problem1().unwrap(), 428690);
  }

  #[test]
  fn check_problem2() {
    assert_eq!(problem2().unwrap(), 3628143500);
  }
}
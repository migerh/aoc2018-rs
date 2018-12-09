use std::collections::VecDeque;

fn play(number_of_players: u32, last_marble: u32) -> u32 {
  let mut circle = VecDeque::with_capacity((2 * last_marble) as usize);
  circle.push_back(0);
  let mut player = 0;
  let mut marble = 0;
  let mut index = 0;
  let mut len = 1;
  let mut scores = vec![0; number_of_players as usize];

  loop {
    player = (player % number_of_players) + 1;
    marble += 1;

    if marble % 100000 == 0 {
      println!("Looking at marble {}", marble);
    }

    if marble % 23 == 0 {
      index = (index + len - 7) % len;
      let score = match circle.remove(index as usize) {
        Some(v) => v,
        None => panic!("Something went wrong")
      };
      scores[(player - 1) as usize] += score + marble;

      len -= 1;

      continue;
    }

    index = (index + 2) % len;
    circle.insert(index as usize, marble);
    len += 1;

    if marble >= last_marble {
      break;
    }
  }

  scores.iter().cloned().fold(0, u32::max)
}

pub fn problem1() -> u32 {
  let players = 405;
  let last_marble = 71700;
  let highscore = play(players, last_marble);

  println!("With {} players and last marble {}, highscore is {}", players, last_marble, highscore);
  highscore
}

pub fn problem2() -> u32 {
  let players = 405;
  let last_marble = 7170000;
  let highscore = play(players, last_marble);

  println!("With {} players and last marble {}, highscore is {}", players, last_marble, highscore);

  highscore
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(play(9, 25), 32);
  }

  #[test]
  fn example2() {
    assert_eq!(play(10, 1618), 8317);
  }

  #[test]
  fn example3() {
    assert_eq!(play(13, 7999), 146373);
  }

  #[test]
  fn example4() {
    assert_eq!(play(17, 1104), 2764);
  }

  #[test]
  fn example5() {
    assert_eq!(play(21, 6111), 54718);
  }

  #[test]
  fn example6() {
    assert_eq!(play(30, 5807), 37305);
  }

  // these two take too long for now

  // #[test]
  // fn check_problem1() {
  //   assert_eq!(problem1(), 428690);
  // }

  // #[test]
  // fn check_problem2() {
  //   assert_eq!(problem2(), 3628143500
  // }
}
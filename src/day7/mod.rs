use std::collections::BTreeSet;
use super::utils::ParseError;

mod common;
mod problem1;
mod problem2;

pub fn problem1() -> Result<String, ParseError> {
  let rules = common::extract_rules()?;
  let mut root_letters = common::find_root(&rules);

  root_letters.sort();

  let mut already_visited = vec![];
  problem1::traverse(&rules, &mut root_letters, &mut already_visited);

  let result: String = already_visited.iter().collect();
  println!("{}", result);

  Ok(result)
}

pub fn problem2() -> Result<i32, ParseError> {
  let rules = common::extract_rules()?;
  let root = common::find_root(&rules);
  let mut letters = BTreeSet::new();
  for r in root {
    letters.insert(r);
  }

  let number_of_elves = 5;
  let mut workers: Vec<Option<problem2::Slot>> = vec![None; number_of_elves];
  let mut tick = -1;
  let mut done = vec![];
  let mut in_work = vec![];

  loop {
    workers = workers.iter().map(|w| {
      match w {
        Some((old_task, 1)) => {
          done.push(*old_task);
          problem2::fetch_new_task(&rules, &done, &mut in_work)
        },
        None => problem2::fetch_new_task(&rules, &done, &mut in_work),
        Some((task, ticks_remaining)) => Some((*task, ticks_remaining - 1))
      }
    }).collect();

    if problem2::all_workers_idle(&workers) {
      break;
    }

    tick += 1;
  }

  println!("Parallel work with 5 elves took {} ticks", tick);

  Ok(tick)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_problem1_works() {
    assert_eq!(problem1().unwrap(), "ABGKCMVWYDEHFOPQUILSTNZRJX");
  }

  #[test]
  fn check_problem2_works() {
    assert_eq!(problem2().unwrap(), 898);
  }
}
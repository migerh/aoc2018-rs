use std::collections::BTreeSet;
use super::utils::ParseError;


mod common {
  use regex::Regex;
  use std::collections::BTreeSet;
  use super::super::utils::{preprocess_input, ParseError};

  pub type Rule = (char, char);

  pub fn get_char(s: &str) -> Result<char, ParseError> {
    match s.chars().next() {
      Some(v) => Ok(v),
      None => Err(ParseError::new("Could not find vertex"))?
    }
  }

  pub fn parse_edge(s: &str) -> Result<Rule, ParseError> {
      lazy_static!{
        static ref REEdge: Regex = Regex::new(r"Step (.) must be finished before step (.) can begin\.").unwrap();
      }

      let cap = match REEdge.captures(s) {
        Some(capture) => capture,
        None => Err(ParseError::new("Could not parse rule"))?
      };

      let from = get_char(&cap[1])?;
      let to = get_char(&cap[2])?;

      Ok((from, to))
  }

  pub fn extract_rules() -> Result<Vec<Rule>, ParseError> {
    let input = include_str!("./data/input.txt");

    let rules = preprocess_input(input)
      .into_iter()
      .map(|v| parse_edge(v))
      .collect::<Result<Vec<Rule>, _>>();

    rules
  }

  pub fn find_root(rules: &Vec<Rule>) -> Vec<char> {
    let mut root_letters = BTreeSet::new();

    for r in rules {
      root_letters.insert(r.0);
    }

    for r in rules {
      root_letters.remove(&r.1);
    }

    root_letters.into_iter().collect()
  }

  pub fn all_dependencies_done(rules: &Vec<Rule>, task: char, done: &Vec<char>) -> bool {
    let mut all_dependencies_visited = true;
    for s in rules {
      if task == s.1 && !done.contains(&s.0) {
        all_dependencies_visited = false;
      }
    }

    all_dependencies_visited
  }
}

mod problem1 {
  use std::collections::BTreeSet;
  use super::common::{Rule, all_dependencies_done};

  fn find_next_steps(rules: &Vec<Rule>, task: char, done: &Vec<char>) -> Vec<char> {
    let mut next_letters = BTreeSet::new();

    for (from, to) in rules {
      if *from == task && all_dependencies_done(rules, *to, done) {
        next_letters.insert(*to);
      }
    }

    next_letters.into_iter().collect()
  }

  fn find_next_letters(rules: &Vec<Rule>, letters: &mut Vec<char>, done: &mut Vec<char>) -> Vec<char> {
    if letters.is_empty() {
      return vec![];
    }

    let c = letters.remove(0);
    done.push(c);

    let mut next_steps = find_next_steps(rules, c, done);
    next_steps.append(letters);
    next_steps.sort();

    next_steps
  }

  pub fn traverse(rules: &Vec<Rule>, letters: &mut Vec<char>, already_visited: &mut Vec<char>) {
    let mut next_steps = find_next_letters(rules, letters, already_visited);
    if next_steps.is_empty() {
      return;
    }
    traverse(rules, &mut next_steps, already_visited);
  }
}

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

mod problem2 {
  use std::collections::BTreeSet;
  use super::common::{Rule, find_root, all_dependencies_done};

  pub type Slot = (char, u8);

  fn get_time(c: char) -> u8 {
    (c as u8) - 4
  }

  fn find_open_tasks(rules: &Vec<Rule>, done: &Vec<char>, in_work: &Vec<char>) -> BTreeSet<char> {
    let mut next_letters = BTreeSet::new();

    let already_assigned = |c| done.contains(&c) || in_work.contains(&c);

    let roots = find_root(rules);
    for r in roots {
      if !already_assigned(r) {
        next_letters.insert(r);
      }
    }

    for (_, t) in rules {
      let task = *t;
      if !already_assigned(task) && all_dependencies_done(rules, task, done) {
        next_letters.insert(task);
      }
    }
    next_letters
  }

  pub fn all_workers_idle(workers: &Vec<Option<Slot>>) -> bool {
    workers.iter().all(|w| *w == None)
  }

  pub fn fetch_new_task(rules: &Vec<Rule>, done: &Vec<char>, in_work: &mut Vec<char>) -> Option<Slot> {
    let open_tasks = find_open_tasks(rules, done, in_work);
    if open_tasks.is_empty() {
      return None;
    }

    let next_letter = *(open_tasks.iter().next().unwrap());
    let time = get_time(next_letter);

    in_work.push(next_letter);

    Some((next_letter, time))
  }
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
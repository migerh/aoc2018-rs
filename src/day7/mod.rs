use regex::Regex;
use std::collections::BTreeSet;

use super::utils::{preprocess_input, ParseError};

type Rule = (char, char);

fn parse_edge(s: &str) -> Result<Rule, ParseError> {
    lazy_static!{
      static ref REEdge: Regex = Regex::new(r"Step (.) must be finished before step (.) can begin\.").unwrap();
    }

    let cap = match REEdge.captures(s) {
      Some(capture) => capture,
      None => Err(ParseError::new("Could not parse rule"))?
    };

    let from = match cap[1].chars().next() {
      Some(v) => v,
      None => Err(ParseError::new("Could not find start edge"))?
    };
    let to = match cap[2].chars().next() {
      Some(v) => v,
      None => Err(ParseError::new("Could not find end edge"))?
    };

    Ok((from, to))
}

fn extract_rules() -> Result<Vec<Rule>, ParseError> {
  let input = include_str!("./data/input.txt");

  let rules = preprocess_input(input)
    .into_iter()
    .map(|v| parse_edge(v))
    .collect::<Result<Vec<Rule>, _>>();

  rules
}

fn find_root(rules: &Vec<Rule>) -> Vec<char> {
  let mut root_letters = BTreeSet::new();

  for r in rules {
    root_letters.insert(r.0);
    root_letters.insert(r.1);
  }

  for r in rules {
    root_letters.remove(&r.1);
  }

  root_letters.into_iter().collect()
}

fn find_next_steps(rules: &Vec<Rule>, step: char, already_visited: &Vec<char>) -> Vec<char> {
  let mut next_letters = BTreeSet::new();

  for r in rules {
    if r.0 == step {
      let mut all_dependencies_visited = true;
      for s in rules {
        if r.1 == s.1 && !already_visited.contains(&s.0) {
          all_dependencies_visited = false;
        }
      }
      if all_dependencies_visited {
        next_letters.insert(r.1);
      }
    }
  }

  next_letters.into_iter().collect()
}

fn find_next_letters(rules: &Vec<Rule>, letters: &mut Vec<char>, already_visited: &mut Vec<char>) -> Vec<char> {
  if letters.len() == 0 {
    return vec![];
  }

  let c = letters.remove(0);
  already_visited.push(c);

  let mut next_steps = find_next_steps(rules, c, already_visited);
  next_steps.append(letters);
  next_steps.sort();

  next_steps
}

fn traverse(rules: &Vec<Rule>, letters: &mut Vec<char>, already_visited: &mut Vec<char>) {
  let mut next_steps = find_next_letters(rules, letters, already_visited);
  if next_steps.len() == 0 {
    return;
  }
  traverse(rules, &mut next_steps, already_visited);
}

pub fn problem1() -> Result<String, ParseError> {
  let rules = extract_rules()?;
  let mut root_letters = find_root(&rules);

  root_letters.sort();

  let mut already_visited = vec![];
  traverse(&rules, &mut root_letters, &mut already_visited);

  let result: String = already_visited.iter().collect();
  println!("{}", result);

  Ok(result)
}

type Slot = (char, u8);

fn get_time(c: char) -> u8 {
  (c as u8) - 4
}

fn find_open_tasks(rules: &Vec<Rule>, done: &Vec<char>, in_work: &Vec<char>) -> BTreeSet<char> {
  let mut next_letters = BTreeSet::new();

  let roots = find_root(rules);

  for r in roots {
    if !done.contains(&r) && !in_work.contains(&r) {
      next_letters.insert(r);
    }
  }

  for r in rules {
    if !done.contains(&r.1) && !in_work.contains(&r.1) {
      let mut all_dependencies_resolved = true;
      for s in rules {
        if r.1 == s.1 && !done.contains(&s.0) {
          all_dependencies_resolved = false;
        }
      }

      if all_dependencies_resolved {
        next_letters.insert(r.1);
      }
    }
  }
  next_letters
}

fn get_idle_workers(workers: &Vec<Option<Slot>>) -> Vec<usize> {
  let mut result = vec![];
  for i in 0..workers.len() {
    if workers[i] == None {
      result.push(i);
    }
  }

  result
}

pub fn problem2() -> Result<i32, ParseError> {
  let rules = extract_rules()?;
  let root = find_root(&rules);
  let mut letters = BTreeSet::new();
  for r in root {
    letters.insert(r);
  }

  let number_of_elves = 5;
  let mut workers: Vec<Option<Slot>> = vec![None; number_of_elves];
  let mut tick = -1;
  let mut done = vec![];
  let mut in_work = vec![];

  while letters.len() > 0 || get_idle_workers(&workers).len() < 5 {
    tick += 1;

    for i in 0..workers.len() {
      let w = workers[i];
      if w != None {
        let v = w.unwrap();
        if v.1 == 1 {
          workers[i] = None;
          done.push(v.0);
        } else {
          workers[i] = Some((v.0, v.1 - 1));
        }
      }
    }

    for i in get_idle_workers(&workers) {
      letters = find_open_tasks(&rules, &done, &in_work);
      if letters.len() == 0 {
        break;
      }

      let next_letter = *(letters.iter().next().unwrap());
      let time = get_time(next_letter);

      in_work.push(next_letter);
      workers[i] = Some((next_letter, time));
    }
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
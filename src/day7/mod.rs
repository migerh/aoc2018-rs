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

fn traverse(rules: &Vec<Rule>, letters: &mut Vec<char>, already_visited: &mut Vec<char>) {
  if letters.len() == 0 {
    return;
  }

  let c = letters.remove(0);
  already_visited.push(c);

  let mut next_steps = find_next_steps(rules, c, already_visited);
  next_steps.append(letters);
  next_steps.sort();
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_problem1_works() {
    assert_eq!(problem1().unwrap(), "ABGKCMVWYDEHFOPQUILSTNZRJX");
  }
}
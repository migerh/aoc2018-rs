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
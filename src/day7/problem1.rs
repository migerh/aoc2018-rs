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
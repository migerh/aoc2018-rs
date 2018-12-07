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
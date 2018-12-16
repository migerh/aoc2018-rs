use super::super::utils::Error;
use std::collections::BTreeMap;
use super::ops::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
  pub registers: [i32; 4],
}

impl State {
  pub fn new(r0: i32, r1: i32, r2: i32, r3: i32) -> State {
    let registers: [i32; 4] = [r0, r1, r2, r3];
    State { registers }
  }

  pub fn apply_all_ops(&self, a: i32, b: i32, c: i32) -> BTreeMap<&'static str, Option<State>> {
    let op_map = op_map();
    let mut map = BTreeMap::new();

    for (op, f) in op_map {
      map.insert(op, f(self.clone(), a, b, c));
    }

    map
  }

  pub fn all_ops() -> Vec<&'static str> {
    op_map().keys().cloned().collect()
  }

  pub fn apply(&self, op: &str, a: i32, b: i32, c: i32) -> Result<Option<State>, Error> {
    let op_map = op_map();

    if op_map.contains_key(op) {
      let f = op_map[op];
      Ok(f(self.clone(), a, b, c))
    } else {
      Err(Error::new("Invalid instruction"))?
    }
  }
}
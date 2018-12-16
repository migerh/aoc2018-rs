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
    let mut map = BTreeMap::new();

    let state = addr(self.clone(), a, b, c);
    map.insert("addr", state);

    let state = addi(self.clone(), a, b, c);
    map.insert("addi", state);

    let state = mulr(self.clone(), a, b, c);
    map.insert("mulr", state);

    let state = muli(self.clone(), a, b, c);
    map.insert("muli", state);

    let state = banr(self.clone(), a, b, c);
    map.insert("banr", state);

    let state = bani(self.clone(), a, b, c);
    map.insert("bani", state);

    let state = borr(self.clone(), a, b, c);
    map.insert("borr", state);

    let state = bori(self.clone(), a, b, c);
    map.insert("bori", state);

    let state = setr(self.clone(), a, b, c);
    map.insert("setr", state);

    let state = seti(self.clone(), a, b, c);
    map.insert("seti", state);

    let state = gtir(self.clone(), a, b, c);
    map.insert("gtir", state);

    let state = gtri(self.clone(), a, b, c);
    map.insert("gtri", state);

    let state = gtrr(self.clone(), a, b, c);
    map.insert("gtrr", state);

    let state = eqir(self.clone(), a, b, c);
    map.insert("eqir", state);

    let state = eqri(self.clone(), a, b, c);
    map.insert("eqri", state);

    let state = eqrr(self.clone(), a, b, c);
    map.insert("eqrr", state);

    map
  }

  pub fn apply(op: i32, a: i32, b: i32, c: i32) {

  }
}
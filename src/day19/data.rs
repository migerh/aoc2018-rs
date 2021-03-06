use super::super::utils::Error;
use super::ops::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
  pub registers: [i128; 6],
  pub ip: i128,
}

impl State {
  pub fn new(ip: i128, r0: i128, r1: i128, r2: i128, r3: i128, r4: i128, r5: i128) -> State {
    let registers: [i128; 6] = [r0, r1, r2, r3, r4, r5];
    State { ip, registers }
  }

  pub fn apply(&self, op: &str, a: i128, b: i128, c: i128) -> Result<State, Error> {
    let op_map = op_map();

    if op_map.contains_key(op) {
      let f = op_map[op];
      match f(self.clone(), a, b, c) {
        Some(mut state) => {
          state.registers[self.ip as usize] += 1;
          Ok(state)
        },
        None => Err(Error::new("Illegal instruction"))?
      }
    } else {
      Err(Error::new("Invalid instruction"))?
    }
  }
}
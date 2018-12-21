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

  pub fn apply(&mut self, op: &str, a: i128, b: i128, c: i128) {
    match op {
      "addr" => {addr(self, a, b, c);},
      "addi" => {addi(self, a, b, c);},
      "mulr" => {mulr(self, a, b, c);},
      "muli" => {muli(self, a, b, c);},
      "banr" => {banr(self, a, b, c);},
      "bani" => {bani(self, a, b, c);},
      "borr" => {borr(self, a, b, c);},
      "bori" => {bori(self, a, b, c);},
      "setr" => {setr(self, a, b, c);},
      "seti" => {seti(self, a, b, c);},
      "gtir" => {gtir(self, a, b, c);},
      "gtri" => {gtri(self, a, b, c);},
      "gtrr" => {gtrr(self, a, b, c);},
      "eqir" => {eqir(self, a, b, c);},
      "eqri" => {eqri(self, a, b, c);},
      "eqrr" => {eqrr(self, a, b, c);},
      _ => {panic!("Illegal instruction!")}
    }
    self.registers[self.ip as usize] += 1;
  }
}
use std::collections::BTreeMap;
use super::data::State;

type OpRef = &'static dyn Fn(&mut State, i128, i128, i128) -> ();

pub fn op_map() -> BTreeMap<&'static str, &'static Fn(&mut State, i128, i128, i128) -> ()> {
  let mut map: BTreeMap<&'static str, OpRef> = BTreeMap::new();

  map.insert("addr", &addr);
  map.insert("addi", &addi);
  map.insert("mulr", &mulr);
  map.insert("muli", &muli);
  map.insert("banr", &banr);
  map.insert("bani", &bani);
  map.insert("borr", &borr);
  map.insert("bori", &bori);
  map.insert("setr", &setr);
  map.insert("seti", &seti);
  map.insert("gtir", &gtir);
  map.insert("gtri", &gtri);
  map.insert("gtrr", &gtrr);
  map.insert("eqir", &eqir);
  map.insert("eqri", &eqri);
  map.insert("eqrr", &eqrr);

  map
}

// addr (add register) stores into register C the result of adding register A and register B.
pub fn addr(state: &mut State, a: i128, b: i128, c: i128) {
  apply_rr(state, a, b, c, |x, y| x + y);
}

// addi (add immediate) stores into register C the result of adding register A and value B.
pub fn addi(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ri(state, a, b, c, |x, y| x + y);
}

// mulr (multiply register) stores into register C the result of multiplying register A and register B.
pub fn mulr(state: &mut State, a: i128, b: i128, c: i128) {
  apply_rr(state, a, b, c, |x, y| x * y);
}

// muli (multiply immediate) stores into register C the result of multiplying register A and value B.
pub fn muli(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ri(state, a, b, c, |x, y| x * y);
}

// banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
pub fn banr(state: &mut State, a: i128, b: i128, c: i128) {
  apply_rr(state, a, b, c, |x, y| x & y);
}

// bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
pub fn bani(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ri(state, a, b, c, |x, y| x & y);
}

// borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
pub fn borr(state: &mut State, a: i128, b: i128, c: i128) {
  apply_rr(state, a, b, c, |x, y| x | y)
}

// bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
pub fn bori(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ri(state, a, b, c, |x, y| x | y);
}

// setr (set register) copies the contents of register A into register C. (Input B is ignored.)
pub fn setr(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ri(state, a, b, c, |x, _| x);
}

// seti (set immediate) stores value A into register C. (Input B is ignored.)
pub fn seti(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ii(state, a, b, c, |x, _| x);
}

// gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
pub fn gtir(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ir(state, a, b, c, |x, y| if x > y { 1 } else { 0 });
}

// gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
pub fn gtri(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ri(state, a, b, c, |x, y| if x > y { 1 } else { 0 });
}

// gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
pub fn gtrr(state: &mut State, a: i128, b: i128, c: i128) {
  apply_rr(state, a, b, c, |x, y| if x > y { 1 } else { 0 });
}

// eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
pub fn eqir(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ir(state, a, b, c, |x, y| if x == y { 1 } else { 0 });
}

// eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
pub fn eqri(state: &mut State, a: i128, b: i128, c: i128) {
  apply_ri(state, a, b, c, |x, y| if x == y { 1 } else { 0 })
}

// eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
pub fn eqrr(state: &mut State, a: i128, b: i128, c: i128) {
  apply_rr(state, a, b, c, |x, y| if x == y { 1 } else { 0 });
}

pub fn valid(v: i128) -> bool {
  0 <= v && v < 6
}

fn apply_rr<F>(mut state: &mut State, a: i128, b: i128, c: i128, f: F)
  where F: Fn(i128, i128) -> i128 {
  if valid(a) && valid(b) && valid(c) {
    state.registers[c as usize] = f(state.registers[a as usize], state.registers[b as usize]);
  }
}

fn apply_ri<F>(mut state: &mut State, a: i128, b: i128, c: i128, f: F)
  where F: Fn(i128, i128) -> i128 {
  if valid(a) && valid(c) {
    state.registers[c as usize] = f(state.registers[a as usize], b);
  }
}

fn apply_ii<F>(mut state: &mut State, a: i128, b: i128, c: i128, f: F)
  where F: Fn(i128, i128) -> i128 {
  if valid(c) {
    state.registers[c as usize] = f(a, b);
  }
}

fn apply_ir<F>(mut state: &mut State, a: i128, b: i128, c: i128, f: F)
  where F: Fn(i128, i128) -> i128 {
  if valid(b) && valid(c) {
    state.registers[c as usize] = f(a, state.registers[b as usize]);
  }
}

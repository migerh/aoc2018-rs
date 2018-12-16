use super::data::State;

// addr (add register) stores into register C the result of adding register A and register B.
pub fn addr(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_rr(state, a, b, c, |x, y| x + y)
}

// addi (add immediate) stores into register C the result of adding register A and value B.
pub fn addi(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ri(state, a, b, c, |x, y| x + y)
}

// mulr (multiply register) stores into register C the result of multiplying register A and register B.
pub fn mulr(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_rr(state, a, b, c, |x, y| x * y)
}

// muli (multiply immediate) stores into register C the result of multiplying register A and value B.
pub fn muli(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ri(state, a, b, c, |x, y| x * y)
}

// banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
pub fn banr(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_rr(state, a, b, c, |x, y| x & y)
}

// bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
pub fn bani(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ri(state, a, b, c, |x, y| x & y)
}

// borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
pub fn borr(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_rr(state, a, b, c, |x, y| x | y)
}

// bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
pub fn bori(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ri(state, a, b, c, |x, y| x | y)
}

// setr (set register) copies the contents of register A into register C. (Input B is ignored.)
pub fn setr(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_rr(state, a, b, c, |x, y| x)
}

// seti (set immediate) stores value A into register C. (Input B is ignored.)
pub fn seti(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ii(state, a, b, c, |x, y| x)
}

// gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
pub fn gtir(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ir(state, a, b, c, |x, y| if x > y { 1 } else { 0 })
}

// gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
pub fn gtri(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ri(state, a, b, c, |x, y| if x > y { 1 } else { 0 })
}

// gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
pub fn gtrr(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_rr(state, a, b, c, |x, y| if x > y { 1 } else { 0 })
}

// eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
pub fn eqir(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ir(state, a, b, c, |x, y| if x == y { 1 } else { 0 })
}

// eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
pub fn eqri(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_ri(state, a, b, c, |x, y| if x == y { 1 } else { 0 })
}

// eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
pub fn eqrr(state: State, a: i32, b: i32, c: i32) -> Option<State> {
  apply_rr(state, a, b, c, |x, y| if x == y { 1 } else { 0 })
}

pub fn valid(v: i32) -> bool {
  0 <= v && v < 4
}

fn apply_rr<F>(mut state: State, a: i32, b: i32, c: i32, f: F) -> Option<State>
  where F: Fn(i32, i32) -> i32 {
  if valid(a) && valid(b) && valid(c) {
    state.registers[c as usize] = f(state.registers[a as usize], state.registers[b as usize]);
    Some(state)
  } else {
    None
  }
}

fn apply_ri<F>(mut state: State, a: i32, b: i32, c: i32, f: F) -> Option<State>
  where F: Fn(i32, i32) -> i32 {
  if valid(a) && valid(b) && valid(c) {
    state.registers[c as usize] = f(state.registers[a as usize], b);
    Some(state)
  } else {
    None
  }
}

fn apply_ii<F>(mut state: State, a: i32, b: i32, c: i32, f: F) -> Option<State>
  where F: Fn(i32, i32) -> i32 {
  if valid(c) {
    state.registers[c as usize] = f(a, b);
    Some(state)
  } else {
    None
  }
}

fn apply_ir<F>(mut state: State, a: i32, b: i32, c: i32, f: F) -> Option<State>
  where F: Fn(i32, i32) -> i32 {
  if valid(b) && valid(c) {
    state.registers[c as usize] = f(a, state.registers[b as usize]);
    Some(state)
  } else {
    None
  }
}

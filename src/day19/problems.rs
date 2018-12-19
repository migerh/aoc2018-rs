use std::collections::BTreeMap;
use super::data::*;
use super::super::utils::{preprocess_input, ParseError, Error};
use regex::{Captures, Regex};

fn instruction_from_capture(capture: &Captures) -> Result<(String, [i128; 3]), ParseError> {
  let instruction = capture[1].to_string();
  let r1 = capture[2].parse::<i128>()?;
  let r2 = capture[3].parse::<i128>()?;
  let r3 = capture[4].parse::<i128>()?;

  Ok((instruction, [r1, r2, r3]))
}

fn parse_ip(s: &str) -> Result<i128, ParseError> {
  lazy_static!{
    static ref RE_ip: Regex = Regex::new(r"#ip (\d+)").unwrap();
  }

  let capture = match RE_ip.captures(s) {
    Some(c) => c,
    None => Err(ParseError::new("Could not parse instruction"))?
  };

  Ok(capture[1].parse::<i128>()?)
}

fn parse_instruction(s: &str) -> Result<(String, [i128; 3]), ParseError> {
  lazy_static!{
    static ref RE_instruction: Regex = Regex::new(r"([a-z]{4}) (\d+) (\d+) (\d+)").unwrap();
  }

  let capture = match RE_instruction.captures(s) {
    Some(c) => c,
    None => Err(ParseError::new("Could not parse instruction"))?
  };

  instruction_from_capture(&capture)
}

pub fn problem1() -> Result<(), Error> {
  let input = include_str!("./data/input.txt");
  let mut program = preprocess_input(input);
  let ip_designation = program.remove(0);

  let ip = parse_ip(ip_designation)?;

  let parsed_program: Vec<(String, [i128; 3])> = program
    .iter()
    .map(|v| parse_instruction(v))
    .collect::<Result<Vec<(String, [i128; 3])>, ParseError>>()?;

  let mut state = State::new(ip, 0, 0, 0, 0, 0, 0);
  let mut i = 0;
  loop {
    let ip = state.registers[state.ip as usize];
    if ip < 0 || ip >= program.len() as i128 {
      break;
    }

    let instruction = &parsed_program[ip as usize];
    // println!("Executing {} = {:?} on {:?}", line, instruction, state);
    let (op, p) = instruction;
    state = state.apply(op.as_str(), p[0], p[1], p[2])?;
    i += 1;
    if i % 1_000_000 == 0 {
      println!("Number of cycles: {}", i);
    }
  }

  println!("final state: {:?}", state);
  let result = state.registers[0];
  println!("Result: {}", result);

  Ok(())
}

pub fn problem2() {
  let mut r: [i128; 6] = [0, 0, 0, 0, 0, 0];
  // #ip 5
  let ip: usize = 5;

  // let result: i64;
  // unsafe {
  //     asm!(
  //         "

  //        1:
  //         jmp 1b

  //        "
  //         : "={r10}"(result)             // Output register
  //         : ""                          // Input registers
  //         : "r10,r11,r12,r13,r14,r15"   // Clobbered registers
  //         : "intel", "volatile"         // Options (intel syntax, don't optimize out)
  //     );
  // }
  // result

  // 00 - addi 5 16 5
  // !ip
  r[5] += 16;

  // 01 - seti 1 1 4
  r[4] = 1;

  // 02 - seti 1 8 2
  r[2] = 1;

  // 03 - mulr 4 2 3
  r[3] = r[4] + r[2];

  // 04 - eqrr 3 1 3
  r[3] = if r[3] == r[1] { 1 } else { 0 };

  // 05 - addr 3 5 5
  // !ip
  r[5] = r[3] + r[5];

  // 06 - addi 5 1 5
  r[5] += 1;

  // 07 - addr 4 0 0
  r[0] += r[4];

  // 08 - addi 2 1 2
  r[2] += 1;

    // 09 - gtrr 2 1 3
    r[3] = if r[2] > r[1] { 1 } else { 0 };

    // 10 - addr 5 3 5
    // !ip
    r[5] += r[3];

    // 11 - seti 2 6 5
    // !ip
    r[5] = 2;

    // 12 - addi 4 1 4
    r[4] += 1;

  if r[2] > r[1] {
    r[4] += 1;
  } else {
    // !ip
    r[5] = 2;
  }

    // 13 - gtrr 4 1 3
    r[3] = if r[4] > r[1] { 1 } else { 0 };

    // 14 - addr 3 5 5
    // !ip
    r[5] += r[3];

    // 15 - seti 1 4 5
    // !ip
    r[5] = 1;

    // 16 - mulr 5 5 5
    // !ip
    r[5] *= r[5];

  if r[4] > r[1] {
    r[5] *= r[5];
  } else {
    r[5] = 1;
  }

  // 17 - addi 1 2 1
  r[1] += 2;

  // 18 - mulr 1 1 1
  r[1] *= r[1];

  // 19 - mulr 5 1 1
  r[1] *= r[5];

  // 20 - muli 1 11 1
  r[1] *= 11;

  // 21 - addi 3 7 3
  r[3] += 7;

  // 22 - mulr 3 5 3
  r[3] *= r[5];

  // 23 - addi 3 8 3
  r[3] += 8;

  // 24 - addr 1 3 1
  r[1] += r[3];

  // 25 - addr 5 0 5
  // !ip
  r[5] += r[0];

  // 26 - seti 0 9 5
  // !ip
  r[5] = 0;

  // 27 - setr 5 8 3
  r[3] = r[5];

  // 28 - mulr 3 5 3
  r[3] *= r[5];

  // 29 - addr 5 3 3
  r[4] += r[5];

  // 20 - mulr 5 3 3
  r[3] *= r[5];

  // 31 - muli 3 14 3
  r[3] *= 14;

  // 32 - mulr 3 5 3
  r[3] *= r[5];

  // 33 - addr 1 3 1
  r[1] += r[3];

  // 34 - seti 0 4 0
  r[0] = 0;

  // 35 - seti 0 3 5
  // !ip
  r[5] = 0;

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_problem1() {
    // assert_eq!(problem2().unwrap(), 674);
  }
}
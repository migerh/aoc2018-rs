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

pub fn problem2() -> Result<(), Error> {
  let result = program();
  println!("Result of reverse engineered program: {}", result);

  Ok(())
}

fn program() -> i128 {
  let mut r: [i128; 6] = [0, 0, 0, 0, 0, 0];

  r[1] += 2;
  r[1] = r[1]*r[1]*19*11 + (r[3] + 7) * 22 + 8;

  if r[0] == 1 {
    r[4] += 29;
    r[1] += 27*28*30*14*32;
    r[0] = 0;
  }

  r[4] = 1;

  loop {
    r[2] = 1;

    loop {
      r[3] = r[4] * r[2];

      if r[3] == r[1] {
        r[0] += r[4];
      }

      r[2] += 1;

      if r[2] > r[1] {
        r[4] += 1;
        break;
      }
    }

    if r[4] > r[1] {
      break;
    }
  }

  r[0]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_problem1() {
    // assert_eq!(problem2().unwrap(), 674);
  }
}
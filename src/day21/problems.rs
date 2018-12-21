use super::data::*;
use super::super::utils::{preprocess_input, ParseError, Error};
use regex::{Captures, Regex};
use std::collections::HashSet;

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

pub fn run() -> Result<(i128, i128), Error> {
  let input = include_str!("./data/input.txt");
  let mut program = preprocess_input(input);
  let ip_designation = program.remove(0);

  let ip = parse_ip(ip_designation)?;

  let parsed_program: Vec<(String, [i128; 3])> = program
    .iter()
    .map(|v| parse_instruction(v))
    .collect::<Result<Vec<(String, [i128; 3])>, ParseError>>()?;

  let mut state = State::new(ip, 0, 0, 0, 0, 0, 0);
  let mut cancelling_numbers = HashSet::new();
  let mut first = None;
  let mut last = 0;
  loop {
    let ip = state.registers[state.ip as usize];
    if ip < 0 || ip >= program.len() as i128 {
      break;
    }

    let instruction = &parsed_program[ip as usize];
    let (op, p) = instruction;
    if state.registers[2] == 28 {
      let cancel = state.registers[4];
      if !cancelling_numbers.insert(cancel) {
        break;
      }
      last = cancel;
      if first.is_none() {
        first = Some(cancel);
      }
    }
    state.apply(op.as_str(), p[0], p[1], p[2]);
  }

  Ok((first.unwrap(), last))
}

pub fn problems() -> Result<(i128, i128), Error> {
  let result = run()?;

  println!("Solution for Problem 1: {}", result.0);
  println!("Solution for Problem 2: {}", result.1);

  Ok(result)
}

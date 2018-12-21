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

pub fn run(mut initial_state: State) -> Result<usize, Error> {
  let input = include_str!("./data/input.txt");
  let mut program = preprocess_input(input);
  let ip_designation = program.remove(0);

  let ip = parse_ip(ip_designation)?;

  let parsed_program: Vec<(String, [i128; 3])> = program
    .iter()
    .map(|v| parse_instruction(v))
    .collect::<Result<Vec<(String, [i128; 3])>, ParseError>>()?;

  let mut state = initial_state;
  let mut i = 0;
  let mut cancelling_numbers = HashSet::new();
  let mut last = 0;
  loop {
    let ip = state.registers[state.ip as usize];
    if ip < 0 || ip >= program.len() as i128 {
      break;
    }

    let instruction = &parsed_program[ip as usize];
    let (op, p) = instruction;
    if state.registers[2] == 28 { //op == "eqrr" && p[0] == 4 && p[1] == 0 && p[2] == 1 {
      let cancel = state.registers[4];
      println!("4: {}, cycles: {}", cancel, i);
      if !cancelling_numbers.insert(cancel) {
        println!("Smallest number that runs the program the longest: {}", last);
        break;
      }
      last = cancel;
    }
    state.apply(op.as_str(), p[0], p[1], p[2])?;
    i += 1;

    if i > 1_000_000_000_000 {
      break;
    }
  }

  Ok(i)
}

pub fn problem1() -> Result<usize, Error> {
  let state = State::new(2, 0, 0, 0, 0, 0, 0);
  let cycles = run(state)?;
  println!("i = {}, cycles = {}", 12213578, cycles);

  Ok(0)
}

pub fn problem2() -> Result<(), Error> {
  Ok(())
}
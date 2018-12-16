use std::collections::BTreeMap;
use super::data::*;
use super::super::utils::{preprocess_input, ParseError, Error};
use regex::{Captures, Regex};

fn state_from_capture(capture: &Captures) -> Result<State, ParseError> {
  let r0 = capture[1].parse::<i32>()?;
  let r1 = capture[2].parse::<i32>()?;
  let r2 = capture[3].parse::<i32>()?;
  let r3 = capture[4].parse::<i32>()?;

  Ok(State::new(r0, r1, r2, r3))
}

fn instruction_from_capture(capture: &Captures) -> Result<[i32; 4], ParseError> {
  let r0 = capture[1].parse::<i32>()?;
  let r1 = capture[2].parse::<i32>()?;
  let r2 = capture[3].parse::<i32>()?;
  let r3 = capture[4].parse::<i32>()?;

  Ok([r0, r1, r2, r3])
}

fn parse_state(s: &str) -> Result<State, ParseError> {
  lazy_static!{
    static ref RE_state: Regex = Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
  }

  let capture = match RE_state.captures(s) {
    Some(c) => c,
    None => Err(ParseError::new("Could not parse state"))?
  };

  state_from_capture(&capture)
}

fn parse_instruction(s: &str) -> Result<[i32; 4], ParseError> {
  lazy_static!{
    static ref RE_instruction: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
  }

  let capture = match RE_instruction.captures(s) {
    Some(c) => c,
    None => Err(ParseError::new("Could not parse instruction"))?
  };

  instruction_from_capture(&capture)
}

fn find_matching_ops(results: &BTreeMap<&'static str, Option<State>>, expected_state: &State) -> Vec<&'static str> {
  let mut valid_ops = vec![];

  for (op, r) in results {
    let state = match r {
      Some(v) => v,
      None => continue
    };

    if state == expected_state {
      valid_ops.push(*op);
    }
  }

  valid_ops
}

fn intersect(a: &Vec<&'static str>, b: &Vec<&'static str>) -> Vec<&'static str> {
  let mut intersection = vec![];

  for entry in a {
    if b.contains(entry) {
      intersection.push(*entry);
    }
  }

  intersection
}

pub fn problem1() -> Result<Vec<Vec<&'static str>>, Error> {
  let input = include_str!("./data/input.txt");
  let lines = preprocess_input(input);

  lazy_static!{
    static ref RE_before: Regex = Regex::new(r"Before:").unwrap();
  }

  let mut line_iterator = lines.iter();
  let mut number_of_samples_match_three_or_more = 0;
  let mut total_samples = 0;
  let mut opcode_map = vec![State::all_ops(); 16];
  while let Some(line) = line_iterator.next() {
    if RE_before.is_match(line) {
      let before_state = parse_state(line)?;
      let instruction_line = match line_iterator.next() {
        Some(l) => l,
        None => Err(ParseError::new("Expected instruction"))?
      };
      let instruction = parse_instruction(instruction_line)?;

      let after_state_line = match line_iterator.next() {
        Some(l) => l,
        None => Err(ParseError::new("Expected instruction"))?
      };
      let expected_after_state = parse_state(after_state_line)?;

      let results = before_state.apply_all_ops(instruction[1], instruction[2], instruction[3]);
      let matching_ops = find_matching_ops(&results, &expected_after_state);

      opcode_map[instruction[0] as usize] = intersect(&opcode_map[instruction[0] as usize], &matching_ops);

      total_samples += 1;
      if matching_ops.len() >= 3 {
        number_of_samples_match_three_or_more += 1;
      }
    }
  }

  println!("Total number of samples: {}", total_samples);
  println!("Of those, {} match three or more ops", number_of_samples_match_three_or_more);


  Ok(opcode_map)
}

pub fn problem2() -> Result<(), Error> {
  let mut opcode_map = problem1()?;

  let mut reduced = false;
  while !reduced {
    let singles = opcode_map
      .iter()
      .enumerate()
      .filter(|(_, value)| value.len() == 1)
      .map(|(index, _)| index)
      .collect::<Vec<usize>>();

    for i in singles {
      let instruction = opcode_map[i as usize][0];
      for k in 0..opcode_map.len() {
        if k != i {
          opcode_map[k] = opcode_map[k].iter().cloned().filter(|v| v != &instruction).collect::<Vec<&str>>();
        }
      }
    }
    reduced = opcode_map.iter().map(|v| v.len()).sum::<usize>() <= 16;
  }

  let opcodes = opcode_map.iter().map(|v| v[0]).collect::<Vec<&str>>();

  let input = include_str!("./data/program.txt");
  let program = preprocess_input(input);

  let mut state = State::new(0, 0, 0, 0);
  for line in program {
    let instruction = parse_instruction(line)?;
    let op = opcodes[instruction[0] as usize];
    state = match state.apply(op, instruction[1], instruction[2], instruction[3])? {
      Some(s) => s,
      None => Err(Error::new("Illegal instruction?"))?
    };
  }

  println!("final state: {:?}", state);
  println!("Result: {}", state.registers[0]);

  Ok(())
}
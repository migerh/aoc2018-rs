use super::utils::{Error, ParseError};
use std::num::ParseIntError;

fn initialize() -> Result<Vec<u8>, ParseError> {
  let input = "209231";

  let result = input
    .chars()
    .map(|c| c.to_string().parse::<u8>())
    .collect::<Result<Vec<u8>, ParseIntError>>()?;

  Ok(result)
}

pub fn problem1() -> Result<(), Error> {
  let mut scores = initialize()?;

  println!("{:?}", scores);

  Ok(())
}
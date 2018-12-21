use super::node::{Directions};

fn preprocess_input(s: &str) -> String {
  s.chars().rev().collect()
}

fn recursive_parse(mut s: &mut String) -> Directions {
  let mut buffer = String::new();
  let mut directions = vec![];
  let mut concats = vec![];
  while let Some(c) = s.pop() {
    match c {
      '$' | '^' => {},
      '(' => {
        if !buffer.is_empty() {
          directions.push(Directions::Content(buffer));
          buffer = String::new();
        }
        let options = recursive_parse(&mut s);
        directions.push(options);
      },
      ')' => {
        if !buffer.is_empty() {
          directions.push(Directions::Content(buffer));
        }
        concats.push(Directions::Concat(directions));
        return Directions::Options(concats);
      },
      '|' => {
        if !buffer.is_empty() {
          directions.push(Directions::Content(buffer));
          buffer = String::new();
        }
        concats.push(Directions::Concat(directions));
        directions = vec![];
      },
      _ => {
        buffer.push(c);
      }
    }
  }

  if buffer.len() > 0 {
    directions.push(Directions::Content(buffer));
  }
  Directions::Concat(directions)
}

pub fn parse(s: &str) -> Directions {
  let mut input = preprocess_input(s);
  recursive_parse(&mut input)
}
use std::iter::Iterator;

use super::utils::ParseError;

#[derive(Debug)]
struct Node {
  pub children: Vec<Node>,
  pub metadata: Vec<i32>,
}

fn next_number<'a, I>(it: &mut I) -> Result<i32, ParseError>
where I: Iterator<Item = &'a str> {
  let val = it.next();
  let result = match val {
    Some(v) => v.parse::<i32>()?,
    None => Err(ParseError::new("Found no value"))?
  };

  Ok(result)
}

fn parse_tree<'a, I>(input: &mut I) -> Result<Node, ParseError> where I: Iterator<Item = &'a str> {
  let number_of_children = next_number(input)?;
  let number_of_metadata = next_number(input)?;

  let mut children = vec![];
  for _i in 0..number_of_children {
    let child = parse_tree(input)?;
    children.push(child);
  }
  let mut metadata = vec![];
  for _i in 0..number_of_metadata {
    metadata.push(next_number(input)?);
  }

  let root = Node { children, metadata };

  Ok(root)
}

fn metadata_sum(root: &Node) -> i32 {
  let mut sum = root.metadata.iter().sum();

  for c in &root.children {
    sum += metadata_sum(c);
  }

  sum
}

pub fn problem1() -> Result<i32, ParseError> {
  let input = include_str!("./data/input.txt");
  let root = parse_tree(&mut input.split(' '))?;
  let sum = metadata_sum(&root);

  println!("Metadata sum: {}", sum);

  Ok(sum)
}

fn value_of_node(root: &Node) -> i32 {
  let mut value = 0;

  if root.children.is_empty() {
    return root.metadata.iter().sum();
  }

  for m in &root.metadata {
    let index = m - 1;
    value += match root.children.get(index as usize) {
      Some(v) => value_of_node(v),
      None => 0
    };
  }

  value
}

pub fn problem2() -> Result<i32, ParseError> {
  let input = include_str!("./data/input.txt");
  let root = parse_tree(&mut input.split(' '))?;

  let result = value_of_node(&root);

  println!("value of root node: {}", result);

  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_problem1_result() {
    assert_eq!(problem1().unwrap(), 46781);
  }

  #[test]
  fn check_problem2_result() {
    assert_eq!(problem2().unwrap(), 21405);
  }
}
use std::collections::BTreeSet;

pub fn problem1() {
  let input = include_str!("./data/input_1-1.txt");
  let result: i32 = input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i32>().unwrap())
    .sum();

  println!("Result of 1-1: {}", result);
}

pub fn problem2() {
  let input = include_str!("./data/input_1-1.txt");
  let numbers = input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i32>().unwrap())
    .collect::<Vec<_>>();

  let mut visited_frequencies = BTreeSet::new();

  let mut result = 0;
  let mut index = 0;
  while !visited_frequencies.contains(&result) {
    visited_frequencies.insert(result);
    result += numbers[index];
    index = (index + 1) % numbers.len();
  }

  println!("First result visited twice: {}", result);
  println!("Number of results generated: {}", visited_frequencies.len());
}
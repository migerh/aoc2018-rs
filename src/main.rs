#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn run() -> Result<(), utils::ParseError> {
  day7::problem2()?;

  if false {
    println!("Past problems:");
    day1::problem1();
    day1::problem2();

    day2::problem1();
    day2::problem2();

    day3::problem1();
    day3::problem2();

    day4::problem1()?;
    day4::problem2()?;

    day5::problem1();
    day5::problem2();

    day6::problem1()?;
    day6::problem2()?;

    day7::problem1()?;
  }

  Ok(())
}

fn main() {
  match run() {
    Err(err) => println!("Error occurred: {}", err),
    _ => {}
  }
}

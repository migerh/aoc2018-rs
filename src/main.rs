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
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod utils;

fn run() -> Result<(), utils::Error> {
  day23::problem2()?;

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
    day7::problem2()?;

    day8::problem1()?;
    day8::problem2()?;

    day9::problem1()?;
    day9::problem2()?;

    day10::problem1()?;
    day10::problem2()?;

    day11::problem1();
    day11::problem2();

    day12::problem1();
    day12::problem2();

    day13::problem1()?;
    day13::problem2()?;

    day14::problems()?;

    day15::problem1()?;
    day15::problem2()?;

    day16::problems::problem1()?;
    day16::problems::problem2()?;

    day17::problems()?;

    day18::problems::problem1()?;
    day18::problems::problem2()?;

    day19::problems::problem1()?;
    day19::problems::problem2()?;

    day20::problems::problems()?;

    day21::problems::problems()?;

    day22::problem1();
    day22::problem2();

    day23::problem1()?;
    day23::problem2()?;

    day24::problems::problem1()?;
    day24::problems::problem2()?;

    day25::problem1()?;
    // day25::problem2()?;
  }

  Ok(())
}

fn main() {
  match run() {
    Err(err) => println!("Error occurred: {}", err),
    _ => {}
  }
}

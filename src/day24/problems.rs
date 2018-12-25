use super::super::utils::Error;
use super::parse::load_groups;
use super::group::{Affiliation, Group};

fn print(groups: &Vec<Group>) {
  println!("Immune system");
  for group in groups.iter().filter(|v| v.affiliation == Affiliation::Immune) {
    println!("{:?}", group);
  }

  println!("Infection");
  for group in groups.iter().filter(|v| v.affiliation == Affiliation::Infection) {
    println!("{:?}", group);
  }
}

pub fn problem1() -> Result<(), Error> {
  let groups = load_groups()?;

  print(&groups);

  Ok(())
}
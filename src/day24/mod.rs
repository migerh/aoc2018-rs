use super::utils::{ParseError, Error};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AttackType {
  Bludgeoning,
  Cold,
  Fire,
  Radiation,
  Slashing,
}

#[derive(Debug, Clone)]
struct Group {
  pub units: u64,
  pub hitpoints: u64,
  pub damage: u64,
  pub attack: AttackType,
  pub initiative: u64,
  pub immunity: HashSet<AttackType>,
  pub weakness: HashSet<AttackType>,
}

fn parse_attack(attack: &str) -> Result<AttackType, ParseError> {
  Ok(match attack {
    "bludgeoning" => AttackType::Bludgeoning,
    "cold" => AttackType::Cold,
    "fire" => AttackType::Fire,
    "radiation" => AttackType::Radiation,
    "slashing" => AttackType::Slashing,
    _ => Err(ParseError::new("Unknown attack type"))?
  })
}

fn parse_weakness_or_immunity(input: &str, re: &Regex) -> Result<HashSet<AttackType>, ParseError> {
    if !re.is_match(input) {
    return Ok(HashSet::new());
  }

  let capture = match re.captures(input) {
    Some(c) => c,
    None => Err(ParseError::new("Could not parse group immunity/weakness"))?
  };

  let mut attack_type = HashSet::new();
  for at in capture.iter().skip(1) {
    if at.is_some() {
      attack_type.insert(parse_attack(at.unwrap().as_str())?);
    }
  }

  Ok(attack_type)
}

fn parse_weakness(input: &str) -> Result<HashSet<AttackType>, ParseError> {
  lazy_static!{
    static ref RE_weak: Regex = Regex::new(r"weak to (\w+),?\s?(\w+)?,?\s?(\w+)?,?\s?(\w+)?").unwrap();
  }

  parse_weakness_or_immunity(input, &RE_weak)
}

fn parse_immunity(input: &str) -> Result<HashSet<AttackType>, ParseError> {
  lazy_static!{
    static ref RE_immune: Regex = Regex::new(r"immune to (\w+),?\s?(\w+)?,?\s?(\w+)?,?\s?(\w+)?").unwrap();
  }

  parse_weakness_or_immunity(input, &RE_immune)
}

fn parse_group(s: &str) -> Result<Option<Group>, ParseError> {
  // 491 units each with 3518 hit points (weak to cold; immune to fire, bludgeoning) with an attack that does 65 radiation damage at initiative 1
  // (weak to cold; immune to fire, bludgeoning)
  lazy_static!{
    static ref RE_group: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (\(.*\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
  }

  if !RE_group.is_match(s) {
    return Ok(None);
  }

  let capture = match RE_group.captures(s) {
    Some(c) => c,
    None => Err(ParseError::new("Could not parse group"))?
  };

  let units = capture[1].parse::<u64>()?;
  let hitpoints = capture[2].parse::<u64>()?;
  let immunities_and_weaknesses = capture[3].to_string();
  let damage = capture[4].parse::<u64>()?;
  let attack = parse_attack(&capture[5])?;
  let initiative = capture[6].parse::<u64>()?;
  let immunity = parse_immunity(immunities_and_weaknesses.as_str())?;
  let weakness = parse_weakness(immunities_and_weaknesses.as_str())?;

  Ok(Some(Group { units, hitpoints, damage, attack, initiative, immunity, weakness }))
}

fn load_groups(input: &str) -> Result<Vec<Group>, Error> {
  let groups = input.split("\n")
    .map(|v| parse_group(v))
    .collect::<Result<Vec<Option<Group>>, ParseError>>()?
    .iter()
    .cloned()
    .filter(|v| v.is_some())
    .map(|v| v.unwrap())
    .collect::<Vec<Group>>();

  Ok(groups)
}

fn load_infections() -> Result<Vec<Group>, Error> {
  let input = include_str!("./data/infection.txt");
  load_groups(input)
}

fn load_immune() -> Result<Vec<Group>, Error> {
  let input = include_str!("./data/immune_system.txt");
  load_groups(input)
}

pub fn problem1() -> Result<(), Error> {
  let immune = load_immune()?;
  let infection = load_infections()?;

  println!("Immune system: {:?}", immune);
  println!("Infections: {:?}", infection);

  Ok(())
}
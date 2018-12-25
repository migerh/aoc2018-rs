use super::group::{Group, AttackType, Affiliation};
use super::super::utils::{ParseError, Error};
use regex::Regex;
use std::collections::HashSet;

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

fn parse_group(s: &str, affiliation: Affiliation) -> Result<Option<Group>, ParseError> {
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

  let id = 0;

  Ok(Some(Group { id, affiliation, units, hitpoints, damage, attack, initiative, immunity, weakness }))
}

fn read_groups(input: &str, affiliation: Affiliation) -> Result<Vec<Group>, Error> {
  let groups = input.split("\n")
    .map(|v| parse_group(v, affiliation.clone()))
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
  // let input = include_str!("./data/ex_infection.txt");
  read_groups(input, Affiliation::Infection)
}

fn load_immune() -> Result<Vec<Group>, Error> {
  let input = include_str!("./data/immune_system.txt");
  // let input = include_str!("./data/ex_immune.txt");
  read_groups(input, Affiliation::Immune)
}

pub fn load_groups() -> Result<Vec<Group>, Error> {
  let mut immune = load_immune()?;
  let mut groups = load_infections()?;
  groups.append(&mut immune);

  for (index, group) in groups.iter_mut().enumerate() {
    group.id = index;
  }

  Ok(groups)
}

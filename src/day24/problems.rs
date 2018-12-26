use super::super::utils::Error;
use super::parse::load_groups;
use super::group::{Affiliation, Group};
use std::collections::HashMap;
use std::cmp::min;


fn _print(groups: &Vec<Group>) {
  println!("Immune system");
  for group in groups.iter().filter(|v| v.affiliation == Affiliation::Immune) {
    println!("{} - {:?}", group.effective_power(), group);
  }

  println!("Infection");
  for group in groups.iter().filter(|v| v.affiliation == Affiliation::Infection) {
    println!("{} - {:?}", group.effective_power(), group);
  }
}

fn target_selection_order(groups: &Vec<Group>) -> Vec<usize> {
  let mut target_selection_groups = groups.clone();
  target_selection_groups.sort_unstable_by(&Group::target_selection_order);

  target_selection_groups
    .iter()
    .map(|v| v.id)
    .collect()
}

fn target_selection_phase(groups: &Vec<Group>) -> HashMap<usize, usize> {
  let selection_order = target_selection_order(&groups);
  let mut already_selected = vec![];
  let mut pairings = HashMap::new();
  for id in selection_order {
    let group = groups.iter().filter(|v| v.id == id).next().unwrap();
    if let Some(target) = group.select_target(&groups, &already_selected) {
      already_selected.push(target);
      pairings.insert(id, target);
    }
  }

  pairings
}

fn determine_attack_order(groups: &Vec<Group>) -> Vec<usize> {
  let mut attack_order = groups.clone();
  attack_order.sort_unstable_by(&Group::attack_order);
  attack_order.iter().map(|v| v.id).collect::<Vec<usize>>()
}

fn attack_phase(groups: &mut Vec<Group>, pairings: &HashMap<usize, usize>) -> u64 {
  let attack_order = determine_attack_order(groups);
  let mut total_units_lost = 0;

  for id in attack_order {
    let group = groups.iter().filter(|v| v.id == id).next().unwrap();
    if group.units == 0 {
      continue;
    }

    if let Some(target) = pairings.get(&id) {
      let (target_index, target_group) = groups.iter().enumerate().filter(|(_, v)| v.id == *target).next().unwrap();
      let damage = group.calculate_damage(target_group);

      let units_lost = damage / target_group.hitpoints;
      total_units_lost += units_lost;
      groups[target_index].units -= min(target_group.units, units_lost);
    }
  }

  total_units_lost
}

fn cleanup(groups: &mut Vec<Group>) {
  let mut remove = vec![];
  for (index, group) in groups.iter().enumerate() {
    if group.units == 0 {
      remove.push(index);
    }
  }

  while let Some(index) = remove.pop() {
    groups.remove(index);
  }
}

fn battle_is_ongoing(groups: &Vec<Group>) -> bool {
  let number_of_immunes = groups.iter().filter(|v| v.affiliation == Affiliation::Immune).count();
  let number_of_infections = groups.iter().filter(|v| v.affiliation == Affiliation::Infection).count();

  number_of_immunes != 0 && number_of_infections != 0
}

fn simulate(boost: u64) -> Result<(Affiliation, u64), Error> {
  let mut groups = load_groups()?;

  for group in &mut groups {
    if group.affiliation == Affiliation::Immune {
      group.damage += boost;
    }
  }

  while battle_is_ongoing(&groups) {
    let pairings = target_selection_phase(&groups);
    let units_lost = attack_phase(&mut groups, &pairings);

    if units_lost == 0 {
      return Ok((Affiliation::Infection, 0));
    }

    cleanup(&mut groups);
  }

  let result: u64 = groups.iter().map(|v| v.units).sum();
  Ok((groups[0].affiliation.clone(), result))
}

pub fn problem1() -> Result<(), Error> {
  let (_, result) = simulate(0)?;
  println!("Result: {}", result);
  Ok(())
}

pub fn problem2() -> Result<(), Error> {
  let mut boost = 50_000;
  let mut last_boost = 0;

  while (last_boost as i64 - boost as i64).abs() > 5 {
    println!("Boost: {}", boost);
    let result = simulate(boost)?;

    let tmp = last_boost;
    last_boost = boost;

    if result.0 == Affiliation::Immune {
      if boost < tmp {
        boost = boost - (tmp - boost) / 2;
      } else {
        boost = (tmp + boost) / 2;
      }
    } else {
      if boost < tmp {
        boost = (tmp + boost) / 2;
      } else {
        boost = boost + (boost - tmp) / 2;
      }
    }
  }

  for b in boost-5..boost+5 {
    let result = simulate(b)?;
    println!("Boost {} yields {:?}", b, result);
    if result.0 == Affiliation::Immune {
      println!("Result: {:?}", result);
      break;
    }
  }

  Ok(())
}
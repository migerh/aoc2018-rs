use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttackType {
  Bludgeoning,
  Cold,
  Fire,
  Radiation,
  Slashing,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Affiliation {
  Immune,
  Infection,
}

#[derive(Debug, Clone)]
pub struct Group {
  pub id: usize,
  pub affiliation: Affiliation,
  pub units: u64,
  pub hitpoints: u64,
  pub damage: u64,
  pub attack: AttackType,
  pub initiative: u64,
  pub immunity: HashSet<AttackType>,
  pub weakness: HashSet<AttackType>,
}

impl Group {
  pub fn effective_power(&self) -> u64 {
    self.units * self.damage
  }

  fn calculate_damage(&self, enemy: &Group) -> u64 {
    let factor = if enemy.immunity.contains(&self.attack) {
      0
    } else if enemy.weakness.contains(&self.attack) {
      2
    } else {
      1
    };

    self.units * self.damage * factor
  }

  // pub fn select_target(&self, enemies: &Vec<Group>) -> usize {
  //   let ranked_enemies = enemies.iter()
  //     .map(|v| )
  //   // todo continue
  //   0
  // }

  pub fn target_selection_order(g: &Group, h: &Group) -> Ordering {
    let gep = g.effective_power();
    let hep = h.effective_power();

    gep.cmp(&hep).then_with(|| g.initiative.cmp(&h.initiative))
  }
}
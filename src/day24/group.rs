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

  pub fn calculate_damage(&self, enemy: &Group) -> u64 {
    let factor = if enemy.immunity.contains(&self.attack) {
      0
    } else if enemy.weakness.contains(&self.attack) {
      2
    } else {
      1
    };

    self.units * self.damage * factor
  }

  pub fn select_target(&self, groups: &Vec<Group>, already_selected: &Vec<usize>) -> Option<usize> {
    let mut available_enemies = groups.iter()
      .filter(|v| v.id != self.id)
      .filter(|v| v.affiliation != self.affiliation)
      .filter(|v| !already_selected.contains(&v.id))
      .cloned()
      .collect::<Vec<Group>>();

    available_enemies.sort_unstable_by(|g, h| {
      let damage_to_g = self.calculate_damage(g);
      let damage_to_h = self.calculate_damage(h);

      damage_to_h.cmp(&damage_to_g)
        .then_with(|| {
          h.effective_power().cmp(&g.effective_power())
        })
        .then_with(|| {
          h.initiative.cmp(&g.initiative)
        })
    });

    if let Some(group) = available_enemies.get(0) {
      Some(group.id)
    } else {
      None
    }
  }

  pub fn target_selection_order(g: &Group, h: &Group) -> Ordering {
    let gep = g.effective_power();
    let hep = h.effective_power();

    hep.cmp(&gep).then_with(|| h.initiative.cmp(&g.initiative))
  }

  pub fn attack_order(g: &Group, h: &Group) -> Ordering {
    h.initiative.cmp(&g.initiative)
  }
}
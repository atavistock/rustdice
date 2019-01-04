#[derive(Default, Debug)]
pub struct DiceSetOptions {
  pub best: u8,
  pub worst: u8,
  pub reroll: u8,
  pub explode: u8,
}

use crate::dietype::Dietype;

#[allow(dead_code)]
impl DiceSetOptions {

  pub fn default() -> DiceSetOptions {
    DiceSetOptions { ..Default::default() }
  }

  pub fn apply(&self, dietype: &Dietype, rolls: &mut Vec<u8>) {
    if self.best > 0 && self.best < rolls.len() as u8 {
      rolls.sort_unstable_by(|a, b| b.cmp(a));
      rolls.resize(self.best as usize, 0);
    }
    if self.worst > 0 && self.worst < rolls.len() as u8 {
      rolls.sort_unstable();
      rolls.resize(self.worst as usize, 0);
    }
    if self.reroll > 0 && self.reroll < dietype.sides {
      for roll in rolls.iter_mut() {
         while *roll <= self.reroll {
          *roll = dietype.roll();
        }
      }
    }
    if self.explode != 0 && self.explode > 1 {
      let mut explode_limit = 20u8;
      let mut new_rolls: Vec<u8> = Vec::new();
      for roll in rolls.iter() {
        new_rolls.push(*roll);
        if *roll <= self.explode {
          let extra_roll = dietype.roll();
          new_rolls.push(extra_roll);
          explode_limit -= 1;
          if explode_limit <= 0 {
            break;
          }
        }
      }
      *rolls = new_rolls;
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn supports_best_options() {
    let options = DiceSetOptions { best: 3, ..Default::default() };
    let mut rolls = vec![3u8, 4, 5, 6];
    let dietype = Dietype { sides: 6 };
    options.apply(&dietype, &mut rolls);
    assert_eq!(rolls, vec![6u8, 5, 4]);
  }

  #[test]
  fn supports_worst_options() {
    let options = DiceSetOptions { worst: 3, ..Default::default() };
    let mut rolls = vec![1u8, 2, 3, 4];
    let dietype = Dietype { sides: 6 };
    options.apply(&dietype, &mut rolls);
    assert_eq!(rolls, vec![1u8, 2, 3]);
  }

  #[test]
  fn support_reroll_options() {
    let options = DiceSetOptions { reroll: 1, ..Default::default() };
    let mut rolls = vec![1u8, 2, 1, 2];
    let dietype = Dietype { sides: 2 };
    options.apply(&dietype, &mut rolls);
    assert_eq!(rolls, vec![2u8, 2, 2, 2]);
  }

  #[test]
  fn support_explode_option() {
    let options = DiceSetOptions { explode: 10, ..Default::default() };
    let mut rolls = vec![10u8, 10];
    let dietype = Dietype { sides: 10 };
    options.apply(&dietype, &mut rolls);
    assert!(rolls.len() >= 4);
  }

}

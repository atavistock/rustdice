#[derive(Default, Debug)]
pub struct DiceSetOptions {
  pub best: u8,
  pub worst: u8,
  pub reroll: u8,
  pub explode: u8,
}

#[allow(dead_code)]
impl DiceSetOptions {

  pub fn default() -> DiceSetOptions {
    DiceSetOptions { ..Default::default() }
  }

  pub fn apply(&self, rolls: &mut Vec<u8>) {
    if self.best > 0 {
      rolls.sort_unstable_by(|a, b| b.cmp(a));
      rolls.resize(self.best as usize, 0);
    }
    if self.worst > 0 {
      rolls.sort_unstable();
      rolls.resize(self.worst as usize, 0);
    }
    if self.reroll > 0 {
      for roll in rolls.iter_mut() {
        println!("Roll? {}", *roll);
        *roll = 3u8;
      }
    }
  }

  // fn apply_reroll_option(&self, rolls)
  //   for roll in rolls.iter_mut() {
  //     while *roll < self.reroll {
  //       roll = &mut self.roll_die();
  //     }
  //   }
  // }

  // fn apply_explode_option(&self, rolls: Vec<u8>) {
  //   // Probably a more efficient way to do this.
  //   let new_rolls = Vec::new();
  //   for roll in rolls.iter() {
  //     let new_roll = *roll;
  //     new_rolls.push(new_roll);
  //     while new_roll >= self.explode {
  //       new_roll = self.roll_die();
  //       new_rolls.push(new_roll);
  //     }
  //   }
  //   rolls = new_rolls;
  // }


}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::dice_set::DiceSet;

  #[test]
  fn supports_best_options() {
    let options = DiceSetOptions { best: 2, ..Default::default() };
    let dice_set = DiceSet::new_with_options("50D2", options).unwrap();
    let value = dice_set.roll();
    assert_eq!(value, 4);
  }

  #[test]
  fn supports_worst_options() {
    let options = DiceSetOptions { worst: 2, ..Default::default() };
    let dice_set = DiceSet::new_with_options("50D2", options).unwrap();
    let value = dice_set.roll();
    assert_eq!(value, 2);
  }

  #[test]
  fn support_reroll_options() {
    let options = DiceSetOptions { reroll: 2, ..Default::default() };
    let dice_set = DiceSet::new_with_options("10D3", options).unwrap();
    let value = dice_set.roll();
    assert_eq!(value, 30);
  }

}

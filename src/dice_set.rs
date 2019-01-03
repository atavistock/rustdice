extern crate regex;

use regex::Regex;
use rand::Rng;

 use crate::dice_options::DiceOptions;

#[derive(Debug)]
struct DiceSet {
  count: u8,
  dietype: u8,
  adjustment: i8,
  options: DiceOptions
}

#[allow(dead_code)]
impl DiceSet {

  pub fn new(dice_str: &str) -> Option<DiceSet> {
    let options = DiceOptions::default();
    DiceSet::new_with_options(dice_str, options)
  }

  pub fn new_with_options(dice_str: &str, options: DiceOptions) -> Option<DiceSet> {
    lazy_static! {
      static ref REGEX: Regex = Regex::new(r"(?xi)
        (?P<count>[0-9]+)D(?P<dietype>[0-9]+)
        (?P<adjustment>[+-][0-9]+)?
      ").unwrap();
    }
    match REGEX.captures(dice_str) {
      Some(captures) => {
        let count = captures["count"].parse::<u8>().unwrap_or(0);
        let dietype = captures["dietype"].parse::<u8>().unwrap_or(0);
        let adjustment = match captures.name("adjustment") {
          Some(matches) => matches.as_str().parse::<i8>().unwrap_or(0i8),
          None => 0i8
        };
        let dice_set = DiceSet {
          count: count,
          dietype: dietype,
          adjustment: adjustment,
          options: options
        };
        Some(dice_set)
      },
      None => None
    }
  }

  // pub fn best(&mut self, value: u8) {
  //   self.best = value;
  // }

  // pub fn worst(&self, value: u8) {
  //   if value < self.count {
  //     self.worst = value;
  //   }
  // }

  // pub fn reroll(&self, value: u8) {
  //   if self.dietype - value > 1 {
  //     self.reroll = value;
  //   }
  // }

  // pub fn explode(&self, value: u8) {
  //   if value > 2 {
  //     self.explode = value;
  //   }
  // }

  pub fn roll(&self) -> i16 {
    let mut rolls: Vec<u8> = Vec::new();
    for _ in 0..self.count {
      let value = self.roll_die();
      rolls.push(value);
    }

    // if self.options.best > 0 {
    //   self.apply_best_option(&mut rolls);
    // }
    // if self.worst > 0 {
    //   self.apply_worst_option(rolls);
    // }
    // if self.reroll > 0 {
    //   self.apply_reroll_option(rolls);
    // }
    // if self.explode != 0 {
    //   self.apply_explode_option(rolls);
    // }

    let mut dice_total : i16 = 0;
    for roll in rolls.iter() {
      dice_total += *roll as i16
    }

    dice_total + self.adjustment as i16
  }

  fn roll_die(&self) -> u8 {
    let mut rng = rand::thread_rng();
    let roll : u8 = rng.gen_range(0, self.dietype);
    roll + 1
  }

  // fn apply_best_option(&self, rolls: &mut Vec<u8>) {
  //   rolls.sort_unstable_by(|a, b| b.cmp(a));
  //   rolls.resize(self.options.best as usize, 0);
  // }

  // fn apply_worst_option(&self, rolls: Vec<u8>) {
  //   rolls.sort_unstable();
  //   rolls.resize(self.worst as usize, 0);
  // }

  // fn apply_reroll_option(&self, rolls: Vec<u8>) {
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

  #[test]
  fn parses_dice_string_with_adjustment() {
    let dice_set = DiceSet::new("3D6-2").unwrap();
    assert_eq!(dice_set.count, 3);
    assert_eq!(dice_set.dietype, 6);
    assert_eq!(dice_set.adjustment, -2);
  }

  #[test]
  fn parses_dice_string_without_adjustment() {
    let dice_set = DiceSet::new("3D6").unwrap();
    assert_eq!(dice_set.count, 3);
    assert_eq!(dice_set.dietype, 6);
    assert_eq!(dice_set.adjustment, 0);
  }

  #[test]
  fn handles_bad_dice_string() {
    let unparsable = DiceSet::new("1F36");
    assert!(unparsable.is_none());
  }

  // Would like to mock out the roll_die method for these tests for
  // more predictable results

  #[test]
  fn die_roll_applies_adjustment() {
    let dice_set = DiceSet::new("1D2+98").unwrap();
    let value = dice_set.roll();
    assert!(value >= 99 && value <= 100);
  }

  #[test]
  fn die_rolls_correct_number_of_dice() {
    let dice_set = DiceSet::new("50D2").unwrap();
    let value = dice_set.roll();
    assert!(value >= 50 && value <= 100);
  }

  //   #[test]
  // fn best_option_applies_correctly() {
  //   let dice_set = DiceSet::new("50D2").unwrap();
  //   dice_set.best(2);
  //   let value = dice_set.roll();
  //   assert_eq!(value, 2);
  // }

}

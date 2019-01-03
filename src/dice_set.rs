extern crate regex;

use regex::Regex;
use rand::Rng;

use crate::dice_options::DiceOptions;

#[derive(Debug)]
pub struct DiceSet {
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

  pub fn roll(&self) -> i16 {
    let mut rolls: Vec<u8> = Vec::new();
    for _ in 0..self.count {
      let value = self.roll_die();
      rolls.push(value);
    }

    self.options.apply(&mut rolls);

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

}

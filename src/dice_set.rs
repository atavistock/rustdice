extern crate regex;

use regex::Regex;
use rand::Rng;

#[derive(Debug)]
struct DiceSet {
  count: u8,
  dietype: u8,
  adjustment: i8
}

impl DiceSet {

  pub fn new(dice_str: &str) -> DiceSet {
    let regex = Regex::new(r"(?P<count>[0-9]+)D(?P<dietype>[0-9]+)(?P<adjustment>[+-][0-9]+)?").unwrap();
    let captures = regex.captures(dice_str).unwrap();

    let count = captures["count"].parse::<u8>().unwrap_or(0);
    let dietype = captures["dietype"].parse::<u8>().unwrap_or(0);
    let adjustment = match captures.name("adjustment") {
      Some(matches) => matches.as_str().parse::<i8>().unwrap_or(0i8),
      None => 0i8
    };

    DiceSet {
      count: count,
      dietype: dietype,
      adjustment: adjustment
    }
  }

  pub fn roll(&self) -> i16 {
    let mut rolls: Vec<u8> = Vec::new();
    for _ in 0..self.count {
      let value = self.roll_die();
      rolls.push(value);
    }

    //apply_options()

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



  // fn apply_options(options: &DiceOptions) {
  // }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_dice_string_with_adjustment() {
    let dice_set = DiceSet::new("3D6-2");
    assert_eq!(dice_set.count, 3);
    assert_eq!(dice_set.dietype, 6);
    assert_eq!(dice_set.adjustment, -2);
  }

  #[test]
  fn parses_dice_string_without_adjustment() {
    let dice_set = DiceSet::new("3D6");
    assert_eq!(dice_set.count, 3);
    assert_eq!(dice_set.dietype, 6);
    assert_eq!(dice_set.adjustment, 0);
  }

  // #[test]
  // fn handles_bad_dice_string() {
  //   let dice_set = DiceSet::new("1F36");
  //   assert_eq!(dice_set.count, 0);
  //   assert_eq!(dice_set.dietype, 0);
  //   assert_eq!(dice_set.adjustment, 0);
  // }



}

#[derive(Default, Debug)]
pub struct DiceOptions {
  best: u8,
  worst: u8,
  reroll: u8,
  explode: u8,
}

#[allow(dead_code)]
impl DiceOptions {

  pub fn default() -> DiceOptions {
    DiceOptions { ..Default::default() }
  }

  pub fn new(best: u8, worst: u8, reroll: u8, explode: u8) -> DiceOptions {
    DiceOptions { best, worst, reroll, explode }
  }

}
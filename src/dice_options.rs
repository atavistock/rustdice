#[derive(Default, Debug)]
struct DiceOptions {
  best: u8,
  worst: u8,
  reroll: u8,
  explode: u8,
}

#[allow(dead_code)]
impl DiceOptions {

  pub fn new(best: u8, worst: u8, reroll: u8, explode: u8) {
    DiceOptions { best, worst, reroll, explode }
  }

}
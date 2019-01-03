use rand::Rng;

#[derive(Debug)]
pub struct Dietype {
  pub sides: u8
}

impl Dietype {

  pub fn roll(&self) -> u8 {
    let mut rng = rand::thread_rng();
    let roll : u8 = rng.gen_range(0, self.sides);
    roll + 1
  }

}


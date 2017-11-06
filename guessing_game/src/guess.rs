pub struct Guess {
value: u32,
}

pub const MAX: u32 = 100;

impl Guess {
  pub fn new(value: u32) -> Guess {
    if value < 1 || value > MAX {
      panic!("Guess must be between 1 and {}, got value {}", MAX, value);
    }

    Guess { value }
  }

  pub fn value(&self) -> u32 {
    self.value
  }
}

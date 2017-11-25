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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_acceptable() {
        let g = Guess::new(1);
        assert_eq!(1, g.value());
    }

    #[test]
    #[should_panic]
    fn test_zero() {
        Guess::new(0);
    }

    #[test]
    #[should_panic]
    fn test_too_big() {
        Guess::new(MAX + 1);
    }

}

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::clone::Clone;

struct CacherSimple<T>
where T: Fn(u32) -> u32
{
calculation: T,
               value: Option<u32>,
}

impl<T> CacherSimple<T>
where T: Fn(u32) -> u32
{
  fn new(calculation: T) -> CacherSimple<T> {
    CacherSimple {
      calculation,
        value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
        None => {
          let v = (self.calculation)(arg);
          self.value = Some(v);
          v
        },
    }
  }
}

struct Cacher<T, U, V>
where T: Fn(&U) -> V, U: Eq + Hash + Clone, V: Clone
{
calculation: T,
               values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where T: Fn(&U) -> V, U: Eq + Hash + Clone, V: Clone
{
  fn new(calculation: T) -> Cacher<T, U, V> {
    Cacher {
      calculation,
        values: HashMap::new(),
    }
  }

  fn value(&mut self, arg: U) -> V {
    if let Some(v) = self.values.get(&arg).map(V::clone) {
      v
    } else {
      let v = (self.calculation)(&arg);
      self.values.insert(arg, v.clone());
      v
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
#[test]
  fn simple_same_value() {
    let mut c = CacherSimple::new(|a| a);

    let v1 = c.value(1);

    assert_eq!(v1, 1);
  }

#[test]
  fn simple_different_values() {
    let mut c = CacherSimple::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 1);
  }

#[test]
  fn hashmap_same_value() {
    let mut c = Cacher::new(|&a| a * 2);

    let v1 = c.value(1);

    assert_eq!(v1, 2);
  }

#[test]
  fn hashmap_different_values() {
    let mut c = Cacher::new(|&a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
  }
}

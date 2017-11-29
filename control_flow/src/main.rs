fn temp_f_to_c(deg_f: f32) -> f32 {
  (deg_f - 32.) * 5. / 9.
}

fn temp_c_to_f(deg_c: f32) -> f32 {
  32. + (deg_c * 9.) / 5.
}

fn fibonacci(n: usize) -> usize {
  let mut fib = vec![0; n + 1];
  if n == 0 {
    return 0
  }
  fib[1] = 1;
  for i in 2..n+1 {
    fib[i] = fib[i - 2] + fib[i - 1];
  }
  fib[n]
}

fn xmas_days(day: usize) {
  if day > 11 {
    println!("There are only 12 days of christmas");
    return
  }
  let days = ["first", "second", "third", "fourth", "fifth", "sixth",
      "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
  let gifts = ["a partridge in a pear tree",
      "two turtle doves",
      "three french hens",
      "four calling birds",
      "five golden rings",
      "six geese a laying",
      "seven swans a swimming",
      "eight maids a milking",
      "nine ladies dancing",
      "ten lords a leaping",
      "eleven pipers piping",
      "twelve drummers drumming"];

  let mut i = day;
  println!("On the {} day of christmas, my true love gave to me:", days[i]);
  loop {
    println!("{}", gifts[i]);
    if i == 0 { break; }
    if i == 1 { print!("and "); }
    i -= 1;
  }
}

fn main() {
  println!("conversions between degrees Fahrenheit and Celsius");
  let temps = vec![0., 32., 70., 100., 212., -55.];
  for temp in temps {
    println!("{} degrees F is {:.2} degrees C", temp, temp_f_to_c(temp));
    println!("{} degrees C is {:.2} degrees F", temp, temp_c_to_f(temp));
  }
  println!("");
  println!("fibonacci numbers");
  for i in 0..10 {
    println!("fibonacci number {} is {}", i, fibonacci(i));
  }
  println!("");
  println!("12 days of christmas");
  xmas_days(0);
  xmas_days(1);
  xmas_days(6);
  xmas_days(11);
}

#[cfg(test)]
mod tests {
  use super::*;

#[test]
  fn test_temp_f_to_c() {
    assert_eq!(0.0, test_temp_f_to_c(32.0));
    assert_eq!(100.0, test_temp_f_to_c(212.0));
    assert_eq!(25.0, test_temp_f_to_c(77.0));
  }


}

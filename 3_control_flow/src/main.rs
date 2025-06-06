fn temp_f_to_c(deg_f: f32) -> f32 {
    (deg_f - 32.) * 5. / 9.
}

fn temp_c_to_f(deg_c: f32) -> f32 {
    32. + (deg_c * 9.) / 5.
}

fn fibonacci(n: usize) -> usize {
    let mut fib = vec![0; n + 1];
    if n == 0 {
        return 0;
    }
    fib[1] = 1;
    for i in 2..n + 1 {
        fib[i] = fib[i - 2] + fib[i - 1];
    }
    fib[n]
}

fn xmas_days(day: usize) {
    if day > 11 {
        println!("There are only 12 days of christmas");
        return;
    }
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear tree",
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
        "twelve drummers drumming",
    ];

    let mut i = day;
    println!(
        "On the {} day of christmas, my true love gave to me:",
        days[i]
    );
    loop {
        println!("{}", gifts[i]);
        if i == 0 {
            break;
        }
        if i == 1 {
            print!("and ");
        }
        i -= 1;
    }
}

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    if number < 5 {
        println!("number {} is less than 5", number);
    } else {
        println!("number {} is not less than 5", number);
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("");
    println!("loop counter");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    println!("");
    println!("loop labels");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("");
    println!("while loop");
    let mut countdown = 10;
    while countdown != 0 {
        println!("{countdown}!");
        countdown -= 1;
    }
    println!("LIFTOFF!!!");

    println!("");
    println!("for loop");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    println!("");
    println!("for loop over range");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    println!("");
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
        assert_eq!(0.0, temp_f_to_c(32.0));
        assert_eq!(100.0, temp_f_to_c(212.0));
        assert_eq!(25.0, temp_f_to_c(77.0));
    }

    #[test]
    fn test_temp_c_to_f() {
        assert_eq!(32.0, temp_c_to_f(0.0));
        assert_eq!(212.0, temp_c_to_f(100.0));
        assert_eq!(77.0, temp_c_to_f(25.0));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(0, fibonacci(0));
        assert_eq!(1, fibonacci(1));
        assert_eq!(1, fibonacci(2));
        assert_eq!(2, fibonacci(3));
        assert_eq!(3, fibonacci(4));
    }
}

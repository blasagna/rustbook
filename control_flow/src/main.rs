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
    // TODO: implement
    ()
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
}

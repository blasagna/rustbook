fn main() {
    print_labeled_measurement(5, 'h');
    scope_block_is_expression();

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn scope_block_is_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// return value
fn plus_one(x: i32) -> i32 {
    x + 1
}
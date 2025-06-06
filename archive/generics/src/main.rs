use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let num_list = vec![34, 50, 25, 100, 65];
    let largest_num = largest(&num_list);
    println!("largest num: {}", largest_num);
    
    let char_list = vec!['y', 'm', 'a', 'A'];
    let largest_char = largest(&char_list);
    println!("largest char: {}", largest_char);
}


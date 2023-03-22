#![allow(dead_code)]
use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

fn main() {
    let number = vec![69, 420];
    println!("{:?}", return_anything(&number));
    print_and_return_anything(&number);
    let meow_meow = Cat {
        name: "Atticus".to_string(),
        age: 6,
    };
    print_and_return_anything(&meow_meow);
    let one_num = 1;
    let two_num = 2;
    compare_and_print("I love monkeys", one_num, two_num);
}

// print generic message and return
fn return_anything<T>(thing: T) -> T {
    println!("Here is your thing:");
    thing
}

// print objects with debug and return
fn print_and_return_anything<T: Debug>(thing: T) -> T {
    println!("Here is your thing: {:?}", thing);
    thing
}

fn compare_and_print<T: Display, N: Display + PartialOrd>(statement: T, num_1: N, num_2: N) {
    println!(
        "{}. Is {} equal to {}? {}",
        statement,
        num_1,
        num_2,
        num_1 == num_2
    );
}

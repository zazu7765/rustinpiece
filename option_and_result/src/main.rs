use std::fmt::Debug;

fn main() {
    let has_no_fifth = vec![1, 2, 3, 4];
    let has_fifth = vec![1, 2, 3, 4, 5];
    custom_option_handler(take_fifth_value(has_fifth));
    custom_option_handler(take_fifth_value(has_no_fifth));
}

fn take_fifth_value(value: Vec<u32>) -> Option<u32> {
    if value.len() < 5 {
        return None;
    }
    Some(value[4])
}

fn custom_option_handler<T: Debug + Copy>(option: Option<T>) {
    match option {
        Some(t) => println!("Found {:?}", t),
        None => println!("Found nothing!"),
    }
}

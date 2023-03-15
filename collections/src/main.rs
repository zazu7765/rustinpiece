fn main() {
//    basic_array_print();
//    arr_slicing();
    string_vector();
}

fn basic_array_print(){
    let array1 = ["one", "two"];
    let myarray = ["a";50];
    println!("Test array: {:?}",array1);
    println!("Char repeating array: {:?}", myarray);
}

fn arr_slicing(){
    // Indexes start at 0 (thank god)
    // Index ranges are exclusive by default
    let one_to_ten = [1,2,3,4,5,6,7,8,9,10];
    let three_to_five = &one_to_ten[2..5];
    let three_to_six = &one_to_ten[2..=5];
    let start_at_two = &one_to_ten[1..];
    let cut_at_five = &one_to_ten[..5];
    let everything_back = &one_to_ten[..];
    println!("Whole array: {:?}\n3 to 5: {:?}\nStarting from 2: {:?}\nCut off at 5: {:?}\nBack to everything: {:?}", one_to_ten,three_to_five,start_at_two,cut_at_five,everything_back);
    println!("Inclusive three to five: {:?}", three_to_six);
}

fn string_vector(){
    let name1 = String::from("Banana");
    let name2 = String::from("Potato");
    let banana = String::from("banana");
    let potato = String::from("potato");
    // vec type not declared in initialization
    let mut food_vec = Vec::new();
    let mut better_food_vec: Vec<String> = Vec::new();
    food_vec.push(name1);
    food_vec.push(name2);
    better_food_vec.push(banana);
    better_food_vec.push(potato);
    println!("String vector no type @ init: {:?}", food_vec);
    println!("String vector typed @ init: {:?}", better_food_vec);
}
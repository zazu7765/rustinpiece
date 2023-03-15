fn main() {
//    basic_print();
//    arr_slicing();
}

fn basic_print(){
    let array1 = ["one", "two"];
    let myarray = ["a";50];
    println!("Test array: {:?}",array1);
    println!("Char repeating array: {:?}", myarray);
}

fn arr_slicing(){
    let one_to_ten = [1,2,3,4,5,6,7,8,9,10];
    let three_to_five = &one_to_ten[2..5];
    let start_at_two = &one_to_ten[1..];
    let cut_at_five = &one_to_ten[..5];
    let everything_back = &one_to_ten[..];
    println!("Whole array: {:?}\n3 to 5: {:?}\nStarting from 2: {:?}\nCut off at 5: {:?}\nBack to everything: {:?}", one_to_ten,three_to_five,start_at_two,cut_at_five,everything_back);
}
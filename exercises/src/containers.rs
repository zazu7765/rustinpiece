mod tests {
    #[test]
    fn assert_array() {
        let array1 = ["one", "two"];
        let myarray = ["a"; 5];
        assert_eq!(array1, ["one", "two"]);
        assert_eq!(myarray, ["a", "a", "a", "a", "a"]);
    }

    #[test]
    fn assert_array_slices() {
        let one_to_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let three_to_five = [3, 4, 5];
        let three_to_six = [3, 4, 5, 6];
        let start_at_two = [2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cut_at_five = [1, 2, 3, 4, 5];
        assert_eq!(one_to_ten, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(three_to_five, &one_to_ten[2..5]);
        assert_eq!(three_to_six, &one_to_ten[2..=5]);
        assert_eq!(start_at_two, &one_to_ten[1..]);
        assert_eq!(cut_at_five, &one_to_ten[..5]);
    }

    #[test]
    fn assert_string_vec() {
        let banana1 = String::from("banana");
        let potato1 = String::from("potato");
        let banana2 = String::from("banana");
        let potato2 = String::from("potato");
        let mut food_vec = Vec::new(); // inferred vector
        let mut better_food_vec: Vec<String> = Vec::new(); // typed vector
        food_vec.push(banana1);
        food_vec.push(potato1);
        better_food_vec.push(banana2);
        better_food_vec.push(potato2);
        assert_eq!(food_vec, better_food_vec);
    }
}

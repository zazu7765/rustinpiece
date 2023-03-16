/* enums are very similar to structs visually but function differently
 * structs are for when you want something AND another thing (MANY THINGS TOGETHER)
 * enums are for when you want something OR another thing (MANY CHOICES TOGETHER)
 */
enum ThingOneOrThingTwo {
    ThingOne,
    ThingTwo,
}
enum PickOnePickTwo {
    PickOne(String),
    PickTwo(String),
}
fn main() {
    let thing = 2;
    let thing_state = return_one_or_two(thing);
    check_thing_state(&thing_state);
    println!("Using Strings!");
    let pick_thing = 1;
    let pick_state = return_one_or_two_string(pick_thing);
    check_thing_state_string(&pick_state);
}

fn return_one_or_two(thing: i8) -> ThingOneOrThingTwo {
    match thing {
        1 => ThingOneOrThingTwo::ThingOne,
        _ => ThingOneOrThingTwo::ThingTwo,
    }
}

fn return_one_or_two_string(thing: i8) -> PickOnePickTwo {
    match thing {
        1 => PickOnePickTwo::PickOne(String::from("ThingOneString")),
        _ => PickOnePickTwo::PickTwo(String::from("ThingTwoString")),
    }
}

fn check_thing_state(state: &ThingOneOrThingTwo) {
    match state {
        ThingOneOrThingTwo::ThingOne => println!("Thing One!"),
        ThingOneOrThingTwo::ThingTwo => println!("Thing Two!"),
    }
}

fn check_thing_state_string(state: &PickOnePickTwo) {
    match state {
        PickOnePickTwo::PickOne(set_string) => println!("{}", set_string),
        PickOnePickTwo::PickTwo(set_string_two) => println!("{}", set_string_two),
    }
}

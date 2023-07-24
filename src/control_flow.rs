pub fn if_else_one(num: i32) -> String {
    let res;
    if num == 1 {
        res = String::from("number is 1");
    } else {
        res = String::from("number is not 1");
    }
    res
}

pub fn if_else_two(num: i32) -> String {
    // extension of if else with logical operators
    if num % 2 == 1 && num > 0 {
        return String::from("positive odd number");
    } else {
        return String::from("not positive odd number");
    }
}

pub fn match_case(num: i32) -> String {
    match num {
        0 => String::from("number is 0"),
        _ => String::from("number is not 0"),
    }
}

pub fn match_as_var(num: i32) -> i32 {
    match num {
        0 => 10,
        1 => 11,
        2 => 12,
        3 => 13,
        4 => 14,
        5 => 15,
        _ => num + 10,
    }
}

pub fn match_string_tuple<'a>(sky: &'a str, temp: &'a str) -> &'a str{
    match (sky, temp) {
        ("cloudy", "cold") => "you should grab a jacket",
        ("cloudy", "warm") => "dont need a jacket!",
        ("clear", "cold") => "bring a jacket and hope for sun",
        ("clear", "warm") => "dont need a jacket!",
        (_, _) => "there's definitely some weather outside",
    }
}

pub fn match_guard(bananas: i32, hungry: bool) -> String{
    match (bananas, hungry) {
        (bananas, hungry) if hungry == false => format!("Not hungry with {} bananas", bananas),
        (bananas, hungry) if bananas == 0 && hungry == true => format!("Hungry but no bananas"),
        _ => format!("Hungry? {}, How many bananas? {}", hungry, bananas),
    }
}

mod tests {
    #[test]
    fn check_if_else_one() {
        let (num, num2) = (5, 1);
        let res = crate::control_flow::if_else_one(num);
        let res2 = crate::control_flow::if_else_one(num2);
        assert_eq!(res, "number is not 1");
        assert_eq!(res2, "number is 1");
    }

    #[test]
    fn check_if_else_two() {
        let (num, num2) = (5, 2);
        let res = crate::control_flow::if_else_two(num);
        let res2 = crate::control_flow::if_else_two(num2);
        assert_eq!(res, "positive odd number");
        assert_eq!(res2, "not positive odd number");
    }
    #[test]
    fn check_match_case() {
        let (num, num2) = (0, 2);
        let res = crate::control_flow::match_case(num);
        let res2 = crate::control_flow::match_case(num2);
        assert_eq!(res, "number is 0");
        assert_eq!(res2, "number is not 0");
    }
    #[test]
    fn check_match_as_var() {
        let (num, num2) = (0, 6);
        let res = crate::control_flow::match_as_var(num);
        let res2 = crate::control_flow::match_as_var(num2);
        assert_eq!(res, 10);
        assert_eq!(res2, 16);
    }
    #[test]
    fn check_match_string_tuple(){
        let (p1, p2) = (("cloudy","cold"),("banana","potato"));
        let res = crate::control_flow::match_string_tuple(p1.0, p1.1);
        let res2 = crate::control_flow::match_string_tuple(p2.0, p2.1);
        assert_eq!(res, "you should grab a jacket");
        assert_eq!(res2, "there's definitely some weather outside");
    }
    #[test]
    fn check_match_guard(){
        let (p1, p2, p3) = ((20, false),(0, true),(30, true));
        let res = crate::control_flow::match_guard(p1.0, p1.1);
        let res2 = crate::control_flow::match_guard(p2.0, p2.1);
        let res3 = crate::control_flow::match_guard(p3.0, p3.1);
        assert_eq!(res, "Not hungry with 20 bananas");
        assert_eq!(res2, "Hungry but no bananas");
        assert_eq!(res3, "Hungry? true, How many bananas? 30");
    }
}

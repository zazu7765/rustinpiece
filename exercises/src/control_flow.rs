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
    fn check_match_case(){
        let (num, num2) = (0, 2);
        let res = crate::control_flow::match_case(num);
        let res2 = crate::control_flow::match_case(num2);
        assert_eq!(res, "number is 0");
        assert_eq!(res2, "number is not 0");
    }
}

fn main() {
//    if_else_one();
//    if_else_two();
//    match_case();
//    match_as_var();
//    string_tuple_match();
//    match_guard();
    rgb_example();
}

fn if_else_one(){
    let my_num = 5;
    if my_num == 7{
        println!("number is 7");
    }
    else if my_num ==6 {
        println!("number is 6");
    }
    else{
        println!("there's definitely a number somewhere");
    }
}

fn if_else_two(){
    let my_num = 5;
    if my_num%2 == 0 && my_num > 0{
        println!("positive odd number");
    }
    else if my_num==6 {
        println!("number is 6");
    }
    else{
        println!("there's definitely a number somewhere");
    }
}

fn match_case(){
    let my_num:u32 = 6;
    match my_num{
        0 => println!("number is 0"),
        1 => println!("number is 1"),
        2 => println!("number is 2"),
        _ => println!("there's definitely a number somewhere"),
    }
}

fn match_as_var(){
    let my_num = 5;
    let match_result = match my_num{
        0=> 10,
        1=> 11,
        2=> 12,
        3=> 13,
        4=> 14,
        5=> 15,
        _ => 69,
    };
    println!("{}", match_result);
}

fn string_tuple_match(){
    let sky = "clear";
    let temperature = "cold";
    match(sky, temperature){
        ("cloudy", "cold")=>println!("you should grab a jacket"),
        ("cloudy", "warm")=>println!("dont need a jacket!"),
        ("clear", "cold")=>println!("bring a jacket and hope for sun"),
        ("clear", "warm")=>println!("dont need a jacket!"),
        (_,_)=>println!("idk there's definitely some weather outside"),
    };
}

fn match_guard(){
    let bananas = 9;
    let hungry = true;
    match(bananas, hungry){
        (bananas, hungry) if hungry==false =>println!("Not hungry with {} bananas :)",bananas),
        (bananas, hungry) if bananas == 0 && hungry == true => println!("Hungry but no bananas :("),
        _ => println!("Hungry? {}, How many bananas? {}", hungry, bananas),
    };
}

fn rgb_example(){
    fn rgb_checker(rgb: (i32, i32, i32)){
        match rgb{
            (r,g,b) if r<10 && g<10 && b<10 =>println!("There's barely any color!"),
            (r,_,_) if r<10 => println!("There's barely any red!"),
            (_,g,_) if g<10 => println!("There's barely any green!"),
            (_,_,b) if b<10 => println!("There's barely any blue!"),
            (r,g,b) if r==g && r==b =>println!("There's an equal amount of each!"),
            _=> println!("theres a good amount of color")
        }
    }
    rgb_checker((10,1,1));
    rgb_checker((200,50,0));
    rgb_checker((50,50,50));
}
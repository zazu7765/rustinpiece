/*
fn print_country(country_name: String){
    println!("{country_name}");
*/
// with return value
/*
fn print_country(country_name: String)-> String{
    println!("{country_name}");
    country_name
}
*/
fn print_country(country_name: &String){
    println!("{country_name}");
}
fn add_country(country_name: &mut String){
    country_name.push_str("-Hungary");
    println!("Now this says {country_name}");
}
fn main() {
    let country = String::from("Canada");
    let mut country_two = String::from("Austria");
    print_country(&country);
    print_country(&country_two);
    add_country(&mut country_two);
}

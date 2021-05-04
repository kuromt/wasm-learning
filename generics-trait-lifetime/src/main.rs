use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Annoucement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("test");
    let string2 = String::from("hogehoge");

    let annouce = String::from("code test");
    let result = longest_with_an_annoucement(&string1, &string2, annouce);
    println!("{}", result);
}
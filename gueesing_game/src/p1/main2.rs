use std::iter::FromIterator;
fn main() {
    let _s1 = give_ownership();
    println!("{}", _s1);
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);
    // println!("{}", s2);
    println!("{}", _s3);
}

fn give_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    let my_chars: Vec<char> = a_string.chars().collect();
    return String::from_iter(my_chars);
}

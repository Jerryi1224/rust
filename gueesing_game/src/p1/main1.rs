fn main() {
    let s1 = String::from("hello");
    // let _s2 = s1;
    // println!("{}, world", s1); // error value s1 borrowed

    let s2 = s1.clone();
    println!("s1= {}, s2= {}", s1, s2);

    takes_ownership(s1);
    let x = 5;
    make_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

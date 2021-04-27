mod display;

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!")
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// can pretty print with {:#?}

fn main() {
    let mut s = gives_ownership();
    println!("after gives_ownership(): {}", s);

    borrows_ownership(&mut s);

    println!("after borrows_ownership(): {}", s);

    takes_ownership(s);
    // println!("{}", s); // should throw compilation error here
}

fn gives_ownership() -> String {
    String::from("Hello, world!")
}

fn takes_ownership(s: String) {
    println!("inside take_ownership(): {}", s);
}

fn borrows_ownership(s: &mut String) {
    s.push_str(" I give up ownership of this string");
}

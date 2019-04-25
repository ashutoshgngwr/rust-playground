fn main() {
    let x = 6;
    let y = x; // deep copy because type is a primitive (i32)

    println!("x = {}, y = {}", x, y); // everything is fine

    let s1 = String::from("Hello, world!");
    let s2 = s1; // shallow copy because type is non-primitive (String)

    // println!("s1 = {}", s1); // things are not fine because s1 reference is dropped
    println!("s2 = {}", s2); // data of s1 is safe in reference s2

    let s1 = String::from("Hello, world!");
    let s2 = s1.clone(); // deep copy using clone method

    println!("s1 = {}, s2 = {}", s1, s2); // things are fine because of deep copy

    let mut s1 = String::from("Hello, world!"); // mutable referene
    {
        let s2 = &s1; // immutable reference
        let s3 = &s1; // multiple immutable reference are fine too
        // let s4 = &mut s1; // not cool because object already has immutable reference

        println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);
    }

    let s2 = &mut s1; // things are fine as long as this is only reference
    // let s3 = &s1; // won't work because object already has mutable reference

    // println!("s1 = {}", s1); // won't work because the reference is dropped after creating a fresh mutable refernce.
    println!("s2 = {}", s2);

    // dangling reference. value is dropped because its owner wasn't transferred
    let s = dangling_reference();
}

fn dangling_reference() -> String { // not returning `&String` for the sake of compilation
    let s = String::from("Hello, world!");
    s // &s // returning ownership `s` would work just fine but `&s`
}

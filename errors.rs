use std::io::{Read, Error};
use std::fs::File;

// unwrap(), expect() extract value if Result is Ok or calls panic if Result is Err
// expect() calls panic with its parameter
fn main() {
    println!("{:?}", read_file_contents("hello-world.rs").expect("file read error"));
    println!("{:?}", read_file_contents("hello.rs").expect("file read error"));
}


// ? returns error to calling function.
// everything is normal, no need to panic. Let rust do it instead!
fn read_file_contents(filepath: &str) -> Result<String, Error> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

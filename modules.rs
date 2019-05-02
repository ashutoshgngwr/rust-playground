// doesn't cover following
// use .. as ..
// nested paths e.g. use std:io::{self, Write}
// glob, use std::io::*
// re-exporting names with - pub use ..;
// so.. this basically covers nothing at all. :/

pub mod nasa {
    pub struct Vehicle {
        id: u8, // private property
        pub name: String
    }

    // impl is never pub
    impl Vehicle {
        pub fn new(name: String, ignition_countdown: u8) -> Vehicle {
            let v = Vehicle { id: 0, name };
            v.ignition_sequence(ignition_countdown);
            v
        }

        fn ignition_sequence(&self, mut countdown: u8) {
            print!("Ignition sequence: ");
            while countdown > 0 {
                print!("{}.. ", countdown);
                countdown -= 1;
            }
            println!();
            println!("Liftoff!!");
        }
    }
}


fn main() {
    let v = nasa::Vehicle::new(String::from("Voyager"), 5);

    // println!("v.id = {}", v.id); // shouldn't work because id is private
    println!("v.name = {}", v.name);
    // v.ignition_sequence(5); // will not work because it is private
}

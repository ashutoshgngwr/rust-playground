// everything variable-y

const EPICNESS: u32 = 1000000;

fn main() {
    constants();
    mutables();
    immutables();
}

fn constants() {
    println!("const EPICNESS = {}", EPICNESS);
}

fn mutables() {
    let mut x: u32 = 1029;
    println!("mutable x = {}", x);
    x = 293;
    println!("mutable x = {} (after re-assignment)", x);
}

fn immutables() {
    // there's really no point of this
    // in fact, the whole program feel freakin' useless right now
    let x = 102;
    println!("immutable x = {}", x);
    let x = 10;
    println!("shadowed x = {} (still immutable, btw)", x);
}

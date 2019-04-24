// funktions because why not!

fn main() {
    // lets not overflow with emotions
    println!("factorial({}) = {}", five(), factorial(five()));
}

fn five() -> u8 {
    5
}

fn factorial(n: u8) -> u8 {
    if n < 2 {
        1
    } else {
        n * factorial(n - 1)
    }
}

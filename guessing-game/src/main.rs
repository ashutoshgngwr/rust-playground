use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let number: u32 = rand::thread_rng().gen_range(1, 100);
    println!("This computer just thought of a number. Find out what is it?");
    take_a_guess(number, 0)
}

fn take_a_guess(number: u32, attempt: u32) {
    print!("Take {} guess: ", if attempt == 0 { "a" } else { "another" });
    io::stdout().flush().ok()
        .expect("Failed to flush stdout!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Unable to read from stdin!");
    let guess: u32 = guess.trim().parse()
        .expect("Unable to parse input!");

    match guess.cmp(&number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win after {} attempts!", attempt);
            return;
        }
    }

    take_a_guess(number, attempt + 1);
}

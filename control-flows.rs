// if-else, loop, for and while

use std::io;

fn main() {
    ifelseifelseif();
    loooop();
    while_loop();
    for_loop();
}

fn ifelseifelseif() {
    println!("ifelseif: Give me an integer!");

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Unable to read line from stdin!");
    let x: u32 = x.trim().parse()
        .expect("Unable to parse uint32!");

    println!("{} is {} 4!", x,
        if x < 4 { "less than" } else if x == 4 { "equal to" } else { "greater than" });
}

fn loooop() {
    println!("loop: Keep feeding me integers! -1 when you're tired");

    loop {
        let mut x = String::new();
        io::stdin().read_line(&mut x)
            .expect("Unable to read line from stdin!");
        let x: i32 = x.trim().parse()
            .expect("Unable to parse int32!");

        if x == -1 {
            break
        }
        println!("You fed me {}!", x);
    }
}

fn while_loop() {
    println!("while_loop: Countdown should be enough!");
    let mut number = 3;

    while number != 0 {
        println!("{}..!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    println!("for_loop: Countdown again!");

    for i in (1..5).rev() {
        println!("{}..!", i);
    }

    println!("BOOOM! .. And you thought it was gonna liftoff!");
}

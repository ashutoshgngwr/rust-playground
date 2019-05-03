use std::io::stdin;
use std::collections::HashMap;

// misc fn:
// hashmap.insert(key, val);
// hashmap.get(&key); // returns Option<&V>

fn main() {
    println!("Enter a string:");
    let mut s = String::new();
    stdin().read_line(&mut s)
        .expect("Cannot read from stdin");

    let mut alpha = HashMap::new();
    for c in s.trim().chars() {
        let count = alpha.entry(c).or_insert(0);
        *count += 1;
    }

    println!("{:?}", alpha);
}

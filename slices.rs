fn main() {
    let s = String::from("HI THERE! JUST TESTING STRING SLICES");

    let fword = first_word(&s);

    // s.free(); // won't work because slices of s are referenced later

    println!("s = {}, fword = {}", s, fword);
    println!("s = {}, s[4..9] = {}", s, &s[4..9]); // pass reference because size is not known at compilation

    // array slices
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("arr[2..8] = {:?}", &arr[2..8]);
}

// use immutable string slice type `&str` instead of `&String`; works for both String and literals
fn first_word(s: &str) -> &str {
    let sbytes = s.as_bytes();
    for (i, &byte) in sbytes.iter().enumerate() {
        if byte == b' ' {
             return &s[..i];
        }
    }

    &s
}

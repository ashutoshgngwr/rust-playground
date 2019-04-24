// (compound) data types and stuff..

fn main() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("x = {}, {}, {}", tuple.0, tuple.1, tuple.2);

    let arr: [u8; 4] = [1, 2, 3, 4];
    println!("arr[2] = {}", arr[2]);

    // this was smaller than I anticipated!
}

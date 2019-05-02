fn main() {
    // let mut v = Vec::new();
    // v.push(3);
    let mut v = vec![3];
    println!("{:?}", v);

    let mut f: i32 = v[0];
    f += 10;
    v[0] += 5;
    println!("v[0] = {}, f = {}", v[0], f);

    v.push(1);
    v.push(4);
    v.push(8);

    // borrowing immutable references to items
    for i in &v {
        println!("{}", i);
    }

    // borrowing mutable references to items
    for i in &mut v {
        *i += 10;
    }

    println!("{:?}", v);
}

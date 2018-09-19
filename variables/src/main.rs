fn mutable_variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn types() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    // integer
    let _dec = 98_222;
    let _hex = 0xff;
    let _oct = 0o77;
    let _bin = 0b1010_0101;
    let _byt = b'A';

    // float
    let _x = 2.0;
    let _y: f32 = 0.5;

    // bool
    let _true = true;
    let _false = false;

    // char
    let _c0 = 'z';
    let _c1 = '😀';

    // tuples
    let _tuple = (500, 6.4, true);
    let (_a, _b, _c) = _tuple;

    // arrays
    let _arr = [1, 2, 3, 4, 5];
}

fn main() {
    mutable_variables();
    shadowing();
    types();
}

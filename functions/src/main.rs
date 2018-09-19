fn main() {
    println!("main()");
    function();
    another_function(plus_one(five()), 1.234);
}

fn function() {
    println!("function()");
}

fn another_function(x: i32, y: f64) {
    println!("another_function({}, {})", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    conditions();
    loops();
}

fn conditions() {
    let num = 3;

    if num < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }

    if num != 0 {
        println!("Number was not zero!");
    }
}

fn loops() {
    // endless loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    assert_eq!(result, 20);

    // for each loop
    let a = [10, 20, 30, 40, 50];
    for el in a.iter() {
        println!("The value is: {}", el);
    }

    for num in (1..6).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!");
}
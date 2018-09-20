#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 59 };

    println!("rect => {:?}", rect);
    println!("area(rect) => {}", area(&rect));
    println!("rect.area() => {}", rect.area());

    let sq = Rectangle::square(16);

    println!("sq => {:?}", sq);
    println!("area(sq) => {}", area(&sq));
    println!("sq.area() => {}", sq.area());
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
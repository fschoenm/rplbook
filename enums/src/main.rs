enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // implementation goes here
    }
}

fn main() {
    // simple enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // enums with value
    let loopback4 = IpAddr::V4(127, 0, 0, 1);
    let loopback6 = IpAddr::V6(String::from("::1"));

    let msg = Message::Write(String::from("Hello World!"));
    msg.call();

    // optional
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

fn route(ip_type: IpAddrKind) {

}
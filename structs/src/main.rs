fn main() {
    // simple structs
    struct User {
        username: String,
        email: String,
        active: bool,
        counter: u32,
    }

    fn create_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            counter: 1,
        }
    }

    let mut user1 = User {
        email: String::from("test@example.com"),
        username: String::from("test"),
        active: true,
        counter: 1,
    };

    let mut user2 = create_user(String::from("admin@example.com"), String::from("admin"));

    user1.counter += 1;
    user2.counter += 1;

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

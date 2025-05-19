struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("Mythos Ma"),
        email: String::from("mythosmayuan@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("Ma Yuan");

    let user2 = build_user(String::from("a"), String::from("value"));

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

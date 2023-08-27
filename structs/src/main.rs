struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main() {
    let mut user1 = User {
        email: String::from("prova@mail.com"),
        username: String::from("prova"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("prova2@gmail.com");

    let _user2 = build_user(String::from("mail@mail.com"), String::from("username2"));

    let _user3 = User{
        email: String::from("email@email.com"),
        .._user2
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

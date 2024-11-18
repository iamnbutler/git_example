use crate::models::user::User;

pub fn login(username: &str, password: &str) -> Option<User> {
    if username == "username" && password == "password" {
        println!("Access granted. User: Alice");
        Some(User::new("Alice")) // Return a User instance when login is successful
    } else {
        println!("Access denied.");
        None // Return None if access is denied
    }
}

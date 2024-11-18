pub struct User {
    pub name: String,
}

impl User {
    pub fn new(username: &str) -> User {
        User {
            name: username.to_string(),
        }
    }
}

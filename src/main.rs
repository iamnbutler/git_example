mod models;
mod services;
mod ui;

fn main() {
    println!("Starting application...");

    ui::auth::show_login_screen();
    let user = services::auth::login("username", "password");
    if let Some(user) = user {
        println!("Welcome, {}!", user.name);
    } else {
        println!("Login failed.");
    }
}

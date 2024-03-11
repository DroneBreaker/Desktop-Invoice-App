use std::collections::HashMap;


fn handle_login() {
    let mut credentials = HashMap::new();

    credentials.insert(String::from("business-tin"), String::from("username"));
}


pub fn main() {
    println!("Welcome to Login");
    handle_login();
}
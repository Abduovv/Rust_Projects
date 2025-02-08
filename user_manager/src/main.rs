struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

enum LoginStatus {
    Success,
    Failure(String),
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_name(user: &mut User, new_name: &str) {
    user.username = String::from(new_name);
}

fn create_user(email: &str, username: &str) -> Result<User, String> {
    if email.contains('@') {
        Ok(User {
            email: String::from(email),
            username: String::from(username),
            active: true,
            sign_in_count: 1,
        })
    } else {
        Err(String::from("Invalid email address"))
    }
}

fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let len = calculate_length(&s);
    println!("String length: {}", len);

    let mut user1 = match create_user("user@example.com", "john_doe") {
        Ok(user) => user,
        Err(e) => {
            println!("Error creating user: {}", e);
            return;
        }
    };

    println!(
        "User created: {} <{}>",
        user1.username, user1.email
    );

    change_name(&mut user1, "john_doe_123");
    println!("Updated username: {}", user1.username);

    let login_attempt = LoginStatus::Failure(String::from("Wrong password"));
    
    match login_attempt {
        LoginStatus::Success => println!("Login successful!"),
        LoginStatus::Failure(reason) => println!("Login failed: {}", reason),
    }

    let maybe_number: Option<i32> = Some(42);
    if let Some(n) = maybe_number {
        println!("The number is: {}", n);
    }
}
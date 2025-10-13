struct User {
    username: String,
    password: String,
}

enum AuthError {
    UserNotFound,
    IncorrectPassword, 
}

fn authenticate<'a>(users: &'a [User], username: &str, password: &str) -> Result<&'a User, AuthError> {
    for user in users {
        if user.username == username {
            if user.password == password {
                return Ok(user);
            } else {
                return Err(AuthError::IncorrectPassword);
            }
        }
    }
    Err(AuthError::UserNotFound)
}


fn main() {
    let users = vec![
        User { username: "alice".into(), password: "1234".into() },
        User { username: "bob".into(), password: "password".into() },
    ];

    match authenticate(&users, "alice", "1234") {
        Ok(user) => println!("Welcome, {}!", user.username),
        Err(AuthError::UserNotFound) => println!("User not found."),
        Err(AuthError::IncorrectPassword) => println!("Wrong password."),
    }

    match authenticate(&users, "charlie", "1234") {
        Ok(user) => println!("Welcome, {}!", user.username),
        Err(AuthError::UserNotFound) => println!("User not found."),
        Err(AuthError::IncorrectPassword) => println!("Wrong password."),
    }
}
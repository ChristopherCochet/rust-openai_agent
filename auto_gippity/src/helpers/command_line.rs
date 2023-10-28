use crossterm::{style::{Color, ResetColor, SetForegroundColor}, ExecutableCommand};
use std::io::{stdin, stdout};

// Get user request
pub fn get_user_response(question : &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in a specific colour
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reste Color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    return user_response.trim().to_string();
}
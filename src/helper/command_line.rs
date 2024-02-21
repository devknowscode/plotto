use std::io::{stdin, stdout};

use crossterm::{
    style::{SetForegroundColor, Stylize},
    ExecutableCommand,
};

use crate::utils::command_color::CommandColor;

pub fn get_user_input(question: &str) -> String {
    let mut stdout = stdout();

    // Print out the prompt in the specific color (blue)
    stdout
        .execute(SetForegroundColor(CommandColor::Blue.get_color()))
        .unwrap();
    println!("");
    println!("{}", question.bold());

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    user_input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_input() {
        get_user_input("This is test from get_user_input() method.");
    }
}

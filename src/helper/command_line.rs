use std::{
    io::{stdin, stdout},
    process::Command,
};

use crossterm::{
    style::{ResetColor, SetForegroundColor, Stylize},
    ExecutableCommand,
};

use crate::{helper::general::EXEC_MAIN_PATH, utils::command_color::CommandColor};

pub enum AgentCommand {
    Info,
    Test,
    Issue,
}

impl AgentCommand {
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout = stdout();

        let agent_color = match self {
            AgentCommand::Info => CommandColor::Blue.get_color(),
            AgentCommand::Test => CommandColor::Yellow.get_color(),
            AgentCommand::Issue => CommandColor::Red.get_color(),
        };

        // Print the agent info
        stdout.execute(SetForegroundColor(agent_color)).unwrap();
        println!(
            "{} {}",
            format!("{}::{}:", "Agent", agent_position).bold(),
            agent_statement.bold()
        );

        // Reset color
        stdout.execute(ResetColor).unwrap();
    }
}

pub fn confirm_safe_code() -> bool {
    let mut stdout = stdout();

    loop {
        // Open project generated
        Command::new("code")
            .arg(EXEC_MAIN_PATH)
            .output()
            .expect("Something went wrong to open source code generated!");

        // Print title command
        stdout
            .execute(SetForegroundColor(CommandColor::Yellow.get_color()))
            .unwrap();
        println!("");
        println!("----------------------------------------------------------");
        println!("WARNING: You are about to run code written entirely by AI.");
        println!("Please review your code and confirm to should be continue.");
        println!("----------------------------------------------------------");

        // Reset color
        stdout.execute(ResetColor).unwrap();

        // Print option to choose
        println!("1. Everything is okay!");
        println!("2. Stop here!");
        println!("");

        // Get user input
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input!");

        user_input = user_input.trim().to_string();

        // Match response
        match user_input.as_str() {
            "1" => return true,
            "2" => return false,
            _ => {
                println!("Invalid input. Please select '1' or '2'");
            }
        }
    }
}

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

    #[test]
    fn test_print_agent_message() {
        let info_command = AgentCommand::Info;
        let test_command = AgentCommand::Test;
        let issue_command = AgentCommand::Issue;

        println!("");
        info_command.print_agent_message("Analyst", "Convert user input to goal");
        println!("");
        test_command.print_agent_message("Tetster", "Implement request in code");
        println!("");
        issue_command.print_agent_message("Debugger", "Fix every bug in code");
        println!("");
    }

    #[test]
    fn test_confirm_safe_code() {
        confirm_safe_code();
    }
}

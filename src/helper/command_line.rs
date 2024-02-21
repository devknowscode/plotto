use std::io::{stdin, stdout};

use crossterm::{
    style::{ResetColor, SetForegroundColor, Stylize},
    ExecutableCommand,
};

use crate::utils::command_color::CommandColor;

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
}

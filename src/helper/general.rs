// import
use openai_dive::v1::resources::chat::{ChatMessage, ChatMessageContent, Role};
use reqwest::Client;
use std::fs;

// mod
use super::command_line::AgentCommand;
use crate::apis::call_request::call_gpt;

// const
pub const CODE_TEMPLATE_PATH: &str =
    "/Users/phat/Desktop/Codespaces/rustc/web_template/src/code_template.rs";
pub const EXEC_MAIN_PATH: &str = "/Users/phat/Desktop/Codespaces/rustc/web_template/src/main.rs";
pub const WEB_SERVER_PROJECT_PATH: &str = "/Users/phat/Desktop/Codespaces/rustc/web_template";
pub const API_SCHEMA_PATH: &str =
    "/Users/phat/Desktop/Codespaces/temp/plotto/schemas/api_schema.json";

pub fn extend_message_to_agent(func: fn(&str) -> &'static str, input: &str) -> ChatMessage {
    let task = func(input);

    // Extend task to to ecourage print out the OUTPUT
    let msg = format!(
        "FUNCTION: {}
        INSTRUCTION: You are a function printer. You ONLY print the results of functions.
        Nothing else. No commantary. Here is the input input to the function {}.
        Print out what the function will return.",
        task, input
    );

    // Return message
    ChatMessage {
        role: Role::System,
        content: ChatMessageContent::Text(msg),
        ..Default::default()
    }
}

pub async fn ai_task_request(
    msg: String,
    agent_position: &str,
    agent_task: &str,
    func: fn(&str) -> &'static str,
) -> String {
    // Print current agent position and operation
    AgentCommand::Info.print_agent_message(agent_position, agent_task);

    // Extend message to get true chat completion
    let extend_message = extend_message_to_agent(func, msg.as_str());

    // Get agent response
    let gpt_response = call_gpt(vec![extend_message]).await;

    gpt_response
}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get code template
pub fn read_code_template() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Get Exec Main
pub fn read_exec_main_code() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read exec main.rs file")
}

// Save new backend code
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs.file");
}

// Save json api endpoint schema
pub fn save_api_endpoint(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write api endpoints to file");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::tasks::analyst::convert_user_input_to_goal;

    use super::*;

    #[test]
    fn test_extend_message_to_agent() {
        let chat_message: ChatMessage = extend_message_to_agent(
            convert_user_input_to_goal,
            "make website to tracking price crypto",
        );

        println!("{:#?}", chat_message);
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        println!("");
        let plotto_response = ai_task_request(
            String::from("Make a website to manage task list"),
            "Analyst",
            "Convert user input to goal",
            convert_user_input_to_goal,
        )
        .await;

        println!("{}", plotto_response);
        println!("");
    }

    #[tokio::test]
    async fn test_check_status_code() {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        let status_code =
            check_status_code(&client, "https://api.binance.com/api/v3/exchangeInfo").await;
        match status_code {
            Ok(code) => {
                println!("{}", code);
            }
            Err(error) => {
                println!("Error checking {}", error);
            }
        }
    }
}

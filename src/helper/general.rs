use openai_dive::v1::{
    endpoints::chat::Chat,
    resources::chat::{ChatMessage, ChatMessageContent, Role},
};

use crate::apis::call_request::call_gpt;

use super::command_line::AgentCommand;

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
    msg: &str,
    agent_position: &str,
    agent_task: &str,
    func: fn(&str) -> &'static str,
) -> String {
    // Print current agent position and operation
    AgentCommand::Info.print_agent_message(agent_position, agent_task);

    // Extend message to get true chat completion
    let extend_message = extend_message_to_agent(func, msg);

    // Get agent response
    let gpt_response = call_gpt(vec![extend_message]).await;

    gpt_response
}

#[cfg(test)]
mod tests {
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
            "Make a website to manage task list",
            "Analyst",
            "Convert user input to goal",
            convert_user_input_to_goal,
        )
        .await;

        println!("{}", plotto_response);
        println!("");
    }
}

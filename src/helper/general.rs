use openai_dive::v1::resources::chat::{ChatMessage, ChatMessageContent, Role};

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
}

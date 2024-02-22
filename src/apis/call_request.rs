use openai_dive::v1::{
    api::Client,
    models::Gpt35Engine,
    resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent},
};
use std::env;

pub async fn call_gpt(messages: Vec<ChatMessage>) -> String {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: Gpt35Engine::Gpt35Turbo16K.to_string(),
        messages,
        ..Default::default()
    };

    let completion = client.chat().create(parameters).await.unwrap();
    match completion.choices[0].message.content.clone() {
        ChatMessageContent::Text(text) => text,
        _ => unreachable!("Unexpected content type"),
    }
}

#[cfg(test)]
mod tests {
    use openai_dive::v1::resources::chat::{ChatMessageContent, Role};

    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let messages: Vec<ChatMessage> = vec![ChatMessage {
            role: Role::User,
            content: ChatMessageContent::Text(
                "This is a test! Give me short response.".to_string(),
            ),
            ..Default::default()
        }];

        let content = call_gpt(messages).await;
        println!("{}", content);
    }
}

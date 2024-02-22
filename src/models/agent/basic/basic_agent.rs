use super::basic_trait::BasicTrait;
use openai_dive::v1::resources::chat::ChatMessage;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Planning,
    Working,
    Testing,
    Done,
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<ChatMessage>,
}

impl BasicTrait for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Planning,
            memory: Vec::from([]),
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_basic_agent() {
        let basic_agent = BasicAgent {
            position: String::from("Analyst"),
            objective: String::from(
                "Gather information and design solution for website development",
            ),
            state: AgentState::Planning,
            memory: vec![],
        };

        dbg!(basic_agent);
    }
}

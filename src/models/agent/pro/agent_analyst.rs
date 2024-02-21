use crate::models::agent::basic::{basic_agent::BasicAgent, basic_trait::BasicTrait};

#[derive(Debug)]
pub struct AgentAnalyst {
    attributes: BasicAgent,
}

impl AgentAnalyst {
    pub fn new() -> Self {
        let attributes = BasicAgent::new(
            String::from("Gather information and design solution for website development"),
            String::from("Project Manager"),
        );

        Self { attributes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_agent_architect() {
        let agent_architect = AgentAnalyst::new();
        println!("{:#?}", agent_architect);
    }
}

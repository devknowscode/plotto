use crate::models::agent::basic::{basic_agent::BasicAgent, basic_trait::BasicTrait};

#[derive(Debug)]
pub struct AgentBackend {
    pub attributes: BasicAgent,
    pub bug_errors: Option<String>,
    pub bug_count: u8,
}

impl AgentBackend {
    pub fn new() -> Self {
        let attributes = BasicAgent::new(
            String::from("Develop backend code for webserver and json database"),
            String::from("Backend Developer"),
        );

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_agent_backend() {
        let agent_backend = AgentBackend::new();
        println!("{:#?}", agent_backend);
    }
}

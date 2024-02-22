use std::time::Duration;

use tokio::time;

use crate::{helper::general::ai_task_request, tasks::analyst::convert_user_input_to_goal};

use super::pro::{
    agent_analyst::AgentAnalyst,
    agent_backend::AgentBackend,
    pro_trait::{GeneralAgent, TaskList},
};

pub struct Manager {
    tasklist: TaskList,
    agents: Vec<Box<dyn GeneralAgent>>,
}

impl Manager {
    pub async fn new(user_input: String) -> Self {
        let description = ai_task_request(
            user_input,
            "Manager",
            "Manage agents who are working for the user",
            convert_user_input_to_goal,
        )
        .await;

        println!("{}", description);

        let tasklist: TaskList = TaskList {
            description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        let agents: Vec<Box<dyn GeneralAgent>> = vec![];

        Self { tasklist, agents }
    }

    pub fn add_agent(&mut self, agent: Box<dyn GeneralAgent>) {
        self.agents.push(agent);
    }

    pub fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentAnalyst::new()));
        self.add_agent(Box::new(AgentBackend::new()));
        // Add more agents in here...
    }

    pub async fn execute(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let _ = agent.execute(&mut self.tasklist).await;
            time::sleep(Duration::from_secs(20)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_managing_agent() {
        let user_input: &str = "need a full stack app that fetches and tracks my fitness progress. Needs to include timezone info from the web.";

        let mut manager = Manager::new(user_input.to_string()).await;

        manager.execute().await;

        dbg!(manager.tasklist);
    }
}

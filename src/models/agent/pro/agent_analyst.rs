use crate::{
    helper::general::ai_task_request,
    models::agent::basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_trait::BasicTrait,
    },
    tasks::analyst::print_project_scope,
};

use super::pro_trait::{ProjectScope, TaskList};

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

    // Define project scope base on print_project_scope task in prompt
    pub async fn define_project_scope(&mut self, tasklist: &mut TaskList) -> ProjectScope {
        let msg = tasklist.description.as_str();
        let gpt_response = ai_task_request(
            msg,
            &self.attributes.position,
            "Print project scope",
            print_project_scope,
        )
        .await;
        // println!("DEBUG::{}", gpt_response);
        let project_scope: ProjectScope = serde_json::from_str(gpt_response.as_str())
            .expect("Failed to decode gpt response from serde_json (project_scope)");
        tasklist.project_scope = Some(project_scope.clone());
        self.attributes.update_state(AgentState::Done);

        println!("{:#?}", project_scope);

        project_scope
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

    #[tokio::test]
    async fn test_define_project_scope() {
        let mut tasklist: TaskList = TaskList {
            description: String::from("build a website that manages task lists"),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        let mut agent_analyst = AgentAnalyst::new();
        let project_scope: ProjectScope = agent_analyst.define_project_scope(&mut tasklist).await;
        println!("{:#?}", project_scope);
    }
}

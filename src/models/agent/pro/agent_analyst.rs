use crate::{
    helper::general::ai_task_request,
    models::agent::basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_trait::BasicTrait,
    },
    tasks::analyst::{print_project_scope, print_site_urls},
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

    // Determine external urls base on print_site_urls task in prompt
    pub async fn determine_external_urls(&mut self, tasklist: &mut TaskList, msg: &str) {
        let gpt_response = ai_task_request(
            msg,
            &self.attributes.position,
            "Print external site urls",
            print_site_urls,
        )
        .await;
        // println!("DEBUG::{}", gpt_response);
        let external_urls: Vec<String> = serde_json::from_str(gpt_response.as_str())
            .expect("Failed to decode gpt response to serde_json (external_urls)");

        println!("{:#?}", external_urls);

        tasklist.external_urls = Some(external_urls);
        self.attributes.update_state(AgentState::Testing);
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

    #[tokio::test]
    async fn test_determine_external_urls() {
        let mut tasklist: TaskList = TaskList {
            description: String::from("build a website that tracks forex and crypto prices"),
            project_scope: Some(ProjectScope {
                is_crud_required: true,
                is_user_login_and_logout: true,
                is_external_urls_required: true,
            }),
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };
        let mut agent_architect = AgentAnalyst::new();

        match tasklist.project_scope {
            Some(ref project_scope) => {
                if project_scope.is_external_urls_required {
                    agent_architect
                        .determine_external_urls(
                            &mut tasklist,
                            "build a website that tracks forex and crypto prices",
                        )
                        .await;
                    assert!(true)
                } else {
                    assert!(false)
                }
            }
            None => assert!(false),
        };
    }
}

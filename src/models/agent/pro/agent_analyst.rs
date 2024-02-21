use std::time::Duration;

use crate::{
    helper::{
        command_line::AgentCommand,
        general::{ai_task_request, check_status_code},
    },
    models::agent::basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_trait::BasicTrait,
    },
    tasks::analyst::{print_project_scope, print_site_urls},
};
use async_trait::async_trait;
use crossterm::style::Stylize;
use reqwest::Client;

use super::pro_trait::{GeneralAgent, ProjectScope, TaskList};

#[derive(Debug)]
pub struct AgentAnalyst {
    attributes: BasicAgent,
}

impl AgentAnalyst {
    pub fn new() -> Self {
        let attributes = BasicAgent::new(
            String::from("Gather information and design solution for website development"),
            String::from("Analyst"),
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

#[async_trait]
impl GeneralAgent for AgentAnalyst {
    fn get_attributes(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(&mut self, tasklist: &mut TaskList) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Done {
            match self.attributes.state {
                AgentState::Planning => {
                    let project_scope: ProjectScope = self.define_project_scope(tasklist).await;

                    // Check project scope has external urls
                    if project_scope.is_external_urls_required {
                        self.determine_external_urls(
                            tasklist,
                            tasklist.description.clone().as_str(),
                        )
                        .await;
                        self.attributes.state = AgentState::Testing;
                    }
                }
                AgentState::Testing => {
                    // Exclude urls require api key
                    let mut exclude_urls: Vec<String> = vec![];

                    // Create request to disconnect after 5 seconds
                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    let urls = tasklist
                        .external_urls
                        .as_ref()
                        .expect("No URLs in task list");

                    // Push url not working to exclude_urls
                    for url in urls {
                        // Print agent command
                        let endpoint_str = format!("Testing URL Endpoint: {}", url);
                        AgentCommand::Test
                            .print_agent_message(&self.attributes.position, &endpoint_str);

                        // Test status url
                        let status_code = check_status_code(&client, url).await;
                        match status_code {
                            Ok(code) => {
                                if code != 200 {
                                    exclude_urls.push(url.clone());
                                    println!("{} ❌", format!("Fail::{}", 404).red().bold());
                                } else {
                                    println!("{} ✅", format!("Pass::{}", 200).green().bold());
                                }
                            }
                            Err(error) => {
                                println!("Error checking {}", error);
                            }
                        }
                    }

                    // Exclude any faulty urls
                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = tasklist
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(url))
                            .cloned()
                            .collect();

                        tasklist.external_urls = Some(new_urls);
                    }

                    self.attributes.state = AgentState::Done;
                }
                _ => self.attributes.state = AgentState::Done,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_agent_analyst() {
        let agent_analyst = AgentAnalyst::new();
        println!("{:#?}", agent_analyst);
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
        let mut agent_analyst = AgentAnalyst::new();

        match tasklist.project_scope {
            Some(ref project_scope) => {
                if project_scope.is_external_urls_required {
                    agent_analyst
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

    #[tokio::test]
    async fn test_execute_agent_analyst() {
        let mut tasklist: TaskList = TaskList {
            description: String::from("build a website that tracks forex and crypto prices"),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        let mut agent_analyst = AgentAnalyst::new();
        let agent_res = agent_analyst.execute(&mut tasklist).await;
        match agent_res {
            Ok(()) => {
                println!("Done work!")
            }
            Err(error) => {
                println!("Error checking: {}", error);
            }
        }
    }

    #[test]
    fn test_print_out() {
        println!("{} ❌", format!("FAIL::{}", 404).red().bold());
        println!("{} ❌", format!("FAIL::{}", 404).red().bold());
        println!("{} ✅", format!("PASS::{}", 200).green().bold());
        println!("{} ✅", format!("PASS::{}", 200).green().bold());
    }
}

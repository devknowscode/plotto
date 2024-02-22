use crate::{
    helper::general::{ai_task_request, read_code_template, save_backend_code},
    models::agent::basic::{basic_agent::BasicAgent, basic_trait::BasicTrait},
    tasks::backend::print_backend_webserver_code,
};

use super::pro_trait::TaskList;

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

    pub async fn initial_backend_code(&mut self, tasklist: &mut TaskList) {
        let code_template = read_code_template();

        let msg: String = format!(
            "CODE TEMPLATE: {} \n PROJECT DESCRIPTION: {} \n",
            code_template, tasklist.description
        );

        let gpt_response = ai_task_request(
            msg,
            &self.attributes.position,
            "Initial backend code",
            print_backend_webserver_code,
        )
        .await;
        // println!("DEBUG::{}", gpt_response);

        // Save main file
        save_backend_code(&gpt_response);
        tasklist.backend_code = Some(gpt_response);
    }
}

#[cfg(test)]
mod tests {
    use crate::models::agent::pro::pro_trait::ProjectScope;

    use super::*;

    #[test]
    fn test_init_agent_backend() {
        let agent_backend = AgentBackend::new();
        println!("{:#?}", agent_backend);
    }

    #[tokio::test]
    async fn test_initial_backend_code() {
        let mut tasklist: TaskList = TaskList {
            description: String::from("build a website that tracks forex and crypto prices"),
            project_scope: Some(ProjectScope {
                is_crud_required: true,
                is_user_login_and_logout: true,
                is_external_urls_required: true,
            }),
            external_urls: Some(vec![
                String::from("https://api.exchangerate-api.com/v4/latest/USD"),
                String::from(
                    "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd",
                ),
            ]),
            backend_code: None,
            api_endpoint_schema: None,
        };

        let mut agent_backend = AgentBackend::new();
        agent_backend.initial_backend_code(&mut tasklist).await;
        println!("{:#?}", tasklist.backend_code);
    }
}

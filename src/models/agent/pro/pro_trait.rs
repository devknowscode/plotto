use async_trait::async_trait;
use serde::Deserialize;

use crate::models::agent::basic::basic_agent::BasicAgent;

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

#[derive(Debug, Clone)]
pub struct TaskList {
    pub description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

#[async_trait]
pub trait GeneralAgent {
    // AgentManager will use to get attributes from agents
    fn get_attributes(&self) -> &BasicAgent;

    // This function will allow agents to execute their logic
    async fn execute(&mut self, tasklist: &mut TaskList) -> Result<(), Box<dyn std::error::Error>>;
}

use std::{
    process::{Command, Stdio},
    time::Duration,
};

use async_trait::async_trait;
use reqwest::Client;
use tokio::time;

use crate::{
    helper::{
        command_line::{confirm_safe_code, AgentCommand},
        general::{
            ai_task_request, check_status_code, read_code_template, read_exec_main_code,
            save_api_endpoint, save_backend_code, WEB_SERVER_PROJECT_PATH,
        },
    },
    models::agent::basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_trait::BasicTrait,
    },
    tasks::{
        backend::{print_backend_webserver_code, print_fixed_code, print_improved_webserver_code},
        tester::print_rest_api_endpoints,
    },
};

use super::pro_trait::{GeneralAgent, RouteObject, TaskList};

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

    pub async fn improve_backend_code(&mut self, tasklist: &mut TaskList) {
        let backend_code = read_exec_main_code();

        let msg = format!(
            "CODE TEMPLATE: {:?} \n PROJECT DESCRIPTION: {:?} \n",
            backend_code, tasklist.description
        );

        let gpt_response = ai_task_request(
            msg,
            &self.attributes.position,
            "Improve backend code",
            print_improved_webserver_code,
        )
        .await;

        println!("DEBUG::{}", gpt_response);

        save_backend_code(&gpt_response);
        tasklist.backend_code = Some(gpt_response);
    }

    pub async fn fix_bug(&mut self, tasklist: &mut TaskList) {
        let backend_code = read_exec_main_code();

        let msg = format!(
            "BROKEN CODE: {:?} \n ERROR BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            backend_code, self.bug_errors
        );

        let gpt_response = ai_task_request(
            msg,
            &self.attributes.position,
            "Fix backend code",
            print_fixed_code,
        )
        .await;

        println!("DEBUG::{}", gpt_response);

        save_backend_code(&gpt_response);
        tasklist.backend_code = Some(gpt_response);
    }

    pub async fn extract_rest_api_endpoints(&mut self) -> String {
        let backend_code: String = read_exec_main_code();

        // Structure message context
        let msg: String = format!("CODE INPUT: {}", backend_code);

        let gpt_response = ai_task_request(
            msg,
            &self.attributes.position,
            "Extract rest api endpoints to schemas",
            print_rest_api_endpoints,
        )
        .await;
        println!("DEBUG::{}", gpt_response);

        gpt_response
    }
}

#[async_trait]
impl GeneralAgent for AgentBackend {
    fn get_attributes(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(&mut self, tasklist: &mut TaskList) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Done {
            match &self.attributes.state {
                AgentState::Planning => {
                    self.initial_backend_code(tasklist).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count > 0 {
                        self.fix_bug(tasklist).await;
                    } else {
                        // self.improve_backend_code(tasklist).await;
                    }
                    self.attributes.state = AgentState::Testing;
                    continue;
                }
                AgentState::Testing => {
                    AgentCommand::Test.print_agent_message(
                        &self.attributes.position,
                        "Confirm code is safe from user...",
                    );

                    let is_continue = confirm_safe_code();

                    if !is_continue {
                        panic!("Better go work on some AI alignment instead...")
                    }

                    // Build and test code generated
                    AgentCommand::Test.print_agent_message(
                        self.attributes.position.as_str(),
                        "Building project...",
                    );

                    // Build code generated
                    // ???
                    let build_backend_server = Command::new("cargo")
                        .arg("build")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .output()
                        .expect("Failed to build backend application");

                    // Determine if build errors
                    if !build_backend_server.status.success() {
                        let error_arr = build_backend_server.stderr;
                        let error_str = String::from_utf8(error_arr).unwrap();

                        // Update error stat
                        self.bug_count += 1;
                        self.bug_errors = Some(error_str);

                        // Exit if too many bugs
                        if self.bug_count > 2 {
                            AgentCommand::Issue.print_agent_message(
                                &self.attributes.position,
                                "Too many bugs found in code...",
                            );
                            panic!("Too many bugs!");
                        }

                        // Pass back to rework
                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    // Build success without errors
                    self.bug_count = 0;
                    AgentCommand::Test.print_agent_message(
                        &self.attributes.position,
                        "Server is built successful",
                    );

                    // Extract api endpoints
                    let gpt_response = self.extract_rest_api_endpoints().await;

                    // Convert api endpoints into values
                    let api_endpoints: Vec<RouteObject> = serde_json::from_str(
                        gpt_response.as_str(),
                    )
                    .expect("Failed to decode gpt response from serde_json (api_endpoints)");

                    // Define "get" and not dynamic endpoints to check
                    let check_endpoints: Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object| {
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        })
                        .cloned()
                        .collect();

                    // Store api endpoints
                    tasklist.api_endpoint_schema = Some(check_endpoints.clone());

                    // Build backend application
                    AgentCommand::Test
                        .print_agent_message(&self.attributes.position, "Starting web server...");

                    // Execute running server
                    let mut run_backend_server = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEB_SERVER_PROJECT_PATH)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("Failed to run backend application");

                    // Let user know testing on server will take place soon
                    AgentCommand::Test.print_agent_message(
                        &self.attributes.position,
                        "Launching test endpoints in 5 seconds...",
                    );
                    time::sleep(Duration::from_secs(5)).await;

                    // Check status code from test
                    for endpoint in check_endpoints {
                        // Print endpoint testing
                        AgentCommand::Test.print_agent_message(
                            self.attributes.position.as_str(),
                            format!("Testing endpoint {}...", endpoint.route).as_str(),
                        );

                        // Create client request with timout 5s
                        let client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .unwrap();

                        // Test endpoint
                        let url = format!("localhost:8080/{}", endpoint.route);
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    AgentCommand::Issue.print_agent_message(
                                        &self.attributes.position,
                                        format!(
                                            "WARNING: Failed to call web server with endpoint {}",
                                            endpoint.route
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                            Err(e) => {
                                // Kill if got error
                                run_backend_server
                                    .kill()
                                    .expect("Failed to kill web server testing.");
                                let err_msg: String = format!("Error checking: {}", e);
                                AgentCommand::Issue
                                    .print_agent_message(&self.attributes.position, &err_msg);
                            }
                        }
                    }
                    save_api_endpoint(&gpt_response);

                    AgentCommand::Test.print_agent_message(
                        &self.attributes.position,
                        "Backend testing complete...",
                    );

                    run_backend_server
                        .kill()
                        .expect("Failed to kill web server testing!");

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

    #[tokio::test]
    async fn test_improved_webserver_code() {
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
            backend_code: Some("```rust\nuse actix_cors::Cors;\nuse actix_web::web::Json;\nuse actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};\nuse async_trait::async_trait;\nuse reqwest::Client as HttpClient;\nuse serde::{Deserialize, Serialize};\n\nuse std::collections::HashMap;\nuse std::fs;\nuse std::io::Write;\nuse std::sync::{Mutex, MutexGuard};\n\n#[derive(Serialize, Deserialize, Debug, Clone)]\nstruct Task {\n    id: u64,\n    name: String,\n    complete: bool,\n}\n\n#[derive(Serialize, Deserialize, Debug, Clone)]\nstruct User {\n    id: u64,\n    username: String,\n    password: String,\n}\n\n#[derive(Serialize, Deserialize, Debug, Clone)]\nstruct Database {\n    tasks: HashMap<u64, Task>,\n    users: HashMap<u64, User>,\n}\n\nimpl Database {\n    fn new() -> Self {\n        Self {\n            tasks: HashMap::new(),\n            users: HashMap::new(),\n        }\n    }\n\n    // TODO CRUD DATA\n    fn insert(&mut self, task: Task) {\n        self.tasks.insert(task.id, task);\n    }\n\n    fn get(&self, id: &u64) -> Option<&Task> {\n        self.tasks.get(id)\n    }\n\n    fn get_all(&self) -> Vec<&Task> {\n        self.tasks.values().collect()\n    }\n\n    fn update(&mut self, task: Task) {\n        self.tasks.insert(task.id, task);\n    }\n\n    fn delete(&mut self, id: &u64) {\n        self.tasks.remove(id);\n    }\n\n    // USER DATA RELATED FUNCTIONS\n    fn insert_user(&mut self, user: User) {\n        self.users.insert(user.id, user);\n    }\n\n    fn get_user_by_name(&self, username: &str) -> Option<&User> {\n        // return user\n        self.users.values().find(|u| u.username == username)\n    }\n\n    // DATABASE SAVING\n    fn save_to_file(&self) -> std::io::Result<()> {\n        // serde_json?\n        let data = serde_json::to_string(&self)?;\n        let mut file = fs::File::create(\"database.json\")?;\n        file.write_all(data.as_bytes())?;\n        Ok(())\n    }\n\n    fn load_file() -> std::io::Result<Self> {\n        let file_content = fs::read_to_string(\"database.json\")?;\n        let db: Self = serde_json::from_str(&file_content)?;\n        Ok(db)\n    }\n}\n\nstruct AppState {\n    db: Mutex<Database>,\n}\n\n// why return impl Responder instead of Responder?\nasync fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {\n    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();\n    db.insert(task.into_inner()); // into_inner?\n    let _ = db.save_to_file();\n    HttpResponse::Ok().finish()\n}\n\nasync fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {\n    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();\n    match db.get(&id.into_inner()) {\n        Some(task) => HttpResponse::Ok().json(task),\n        None => HttpResponse::NotFound().finish(),\n    }\n}\n\nasync fn read_all_task(app_state: web::Data<AppState>) -> impl Responder {\n    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();\n    let tasks = db.get_all();\n    HttpResponse::Ok().json(tasks)\n}\n\nasync fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {\n    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();\n    db.update(task.into_inner());\n    let _ = db.save_to_file();\n    HttpResponse::Ok().finish()\n}\n\nasync fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {\n    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();\n    db.delete(&id.into_inner());\n    let _ = db.save_to_file();\n    HttpResponse::Ok().finish()\n}\n\nasync fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {\n    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();\n    db.insert_user(user.into_inner());\n    let _ = db.save_to_file();\n    HttpResponse::Ok().finish()\n}\n\nasync fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {\n    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();\n    match db.get_user_by_name(&user.username) {\n        Some(stored_user) if stored_user.password == user.password => {\n            HttpResponse::Ok().body(\"Logged in!\")\n        }\n        _ => HttpResponse::BadRequest().body(\"Invalid username or password\"),\n    }\n}\n\n#[actix_web::main]\n// what is Result<()>\nasync fn main() -> std::io::Result<()> {\n    let db = match Database::load_file() {\n        Ok(db) => db,\n        Err(_) => Database::new(),\n    };\n\n    let data = web::Data::new(AppState { db: Mutex::new(db) });\n\n    // what is `move` used for in here?\n    HttpServer::new(move || {\n        App::new()\n            .wrap(\n                Cors::permissive()\n                    .allowed_origin_fn(|origin, _req_head| {\n                        origin.as_bytes().starts_with(b\"http://localhost\") || origin == \"null\"\n                    })\n                    .allowed_methods(vec![\"GET\", \"POST\", \"PUT\", \"DELETE\"])\n                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])\n                    .allowed_header(header::CONTENT_TYPE)\n                    .supports_credentials()\n                    .max_age(3600),\n            )\n            .app_data(data.clone())\n            .route(\"/task\", web::post().to(create_task))\n            .route(\"/task\", web::get().to(read_all_task))\n            .route(\"/task/{id}\", web::get().to(read_task))\n            .route(\"/task\", web::put().to(update_task))\n            .route(\"/task/{id}\", web::delete().to(delete_task))\n            .route(\"/register\", web::post().to(register))\n            .route(\"/login\", web::post().to(login))\n    })\n    .bind(\"127.0.0.1:8080\")?\n    .run()\n    .await\n}\n```".to_string()),
            api_endpoint_schema: None,
        };

        let mut agent_backend = AgentBackend::new();
        agent_backend.improve_backend_code(&mut tasklist).await;
    }

    #[tokio::test]
    async fn test_execute_agent_backend() {
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
        agent_backend
            .execute(&mut tasklist)
            .await
            .expect("Failed to execute backend developer!");
    }
}

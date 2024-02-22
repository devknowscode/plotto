use crate::{
    helper::general::{
        ai_task_request, read_code_template, read_exec_main_code, save_backend_code,
    },
    models::agent::basic::{basic_agent::BasicAgent, basic_trait::BasicTrait},
    tasks::backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    },
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
}

use lib::function_to_string;

#[function_to_string]
pub fn print_rest_api_endpoints(_code_input: &str) {
    /// INPUT: Takes in Rust webserver CODE_INPUT based on actix-web
    /// FUNCTION: Prints out the JSON schema for url endpoints and their respective types
    /// LOGIC: Script analyses all code and can categorize into the following object keys:
    ///   "route": This represents the url path of the endpoint
    ///   "is_route_dynamic": if a route has curly braces in it such as {symbol} or {id} as an example, then this will be set to true
    ///   "method": This represents the method being called
    ///   "request_body": This represents the body of a post method request
    ///   "response": This represents the output based upon the structs in the code and understanding the functions
    /// IMPORTANT: Only prints out the JSON schema. No commentary or anything else.
    /// MUST READ: All keys are strings. Even bool should be wrapped in double quotes as "bool"
    /// EXAMPLE:
    /// INPUT_CODE:
    /// ...
    /// pub struct Item {
    ///   pub id: u64,
    ///   pub name: String,
    ///   pub completed: bool,
    /// }
    /// pub struct User {
    ///   pub id: u64,
    ///   pub username: String,
    ///   pub password: String,
    /// }
    /// ...
    /// HttpServer::new(move || {
    ///   App::new()
    ///       .app_data(data.clone())
    ///       .route("/item", web::post().to(create_item))
    ///       .route("/item/{id}", web::get().to(read_item))
    ///       .route("/item/{id}", web::put().to(update_item))
    ///       .route("/item/{id}", web::delete().to(delete_item))
    ///       .route("/signup", web::post().to(signup))
    ///       .route("/crypto", web::get().to(crypto))
    /// PRINTS JSON FORMATTED OUTPUT:
    /// [
    ///   {
    ///     "route": "/item/{id}",
    ///     "is_route_dynamic": "true",
    ///     "method": "get"
    ///     "request_body": "None",
    ///     "response": {
    ///       "id": "number",
    ///       "name": "string",
    ///       "completed": "bool",
    ///     }
    ///   },
    ///   {
    ///     "route": "/item",
    ///     "is_route_dynamic": "false",
    ///     "method": "post",
    ///     "request_body": {
    ///       "id": "number",
    ///       "name": "string",
    ///       "completed": "bool",
    ///     },
    ///     "response": "None"
    ///   },
    ///   {
    ///     "route": "/item/{id}",
    ///     "is_route_dynamic": "true",
    ///     "method": "delete",
    ///     "request_body": "None",
    ///     "response": "None"
    ///   },
    ///   {
    ///     "route": "/crypto",
    ///     "is_route_dynamic": "false",
    ///     "method": "get",
    ///     "request_body": "None",
    ///     "response": "not_provided"
    ///   },
    ///   ... // etc
    /// ]
    println!(OUTPUT)
}

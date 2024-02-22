use helper::command_line::get_user_input;

use crate::models::agent::manager::Manager;

mod apis;
mod helper;
mod models;
mod tasks;
mod utils;

#[tokio::main]
async fn main() {
    let user_input = get_user_input("What are we building today?");
    let mut manager = Manager::new(user_input).await;
    manager.execute().await;
    println!("Done work!!!");
}

#[cfg(test)]
mod tests {
    use lib::function_to_string;

    #[function_to_string]
    fn this_is_test(_param: &str) {
        /// Comment in this test
        println!("{}", OUTPUT);
    }

    #[test]
    fn test_function_to_string() {
        let fn_str = this_is_test("Blah blah blah...");
        println!("{:#?}", fn_str);
    }
}

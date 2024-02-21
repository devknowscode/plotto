use helper::command_line::get_user_input;

mod apis;
mod helper;
mod utils;

fn main() {
    get_user_input("What are we building today?");
}

use lib::function_to_string;

#[function_to_string]
pub fn print_backend_webserver_code(_project_description_and_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
    /// IMPORTANT: The backend code is ONLY an example. If the Project Description requires it, make as many changes as you like.
    /// IMPORTANT: You do not need to follow the backend code exactly. Write functions that make sense for the users request if required.
    /// FUNCTION: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the purpose in the PROJECT_DESCRIPTION
    /// IMPORTANT: The following libraries are already installed
    ///   reqwest, serde, serde_json, tokio, actix-web, async-trait, actix_cors
    /// No other external libraries should be used. Write functions that fit with the description from the PROJECT_DESCRIPTION
    /// OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code.
    println!(OUTPUT)
}

#[function_to_string]
pub fn print_improved_webserver_code(_project_description_and_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
    /// FUNCTION: Performs the following tasks:
    ///   1. Removes any bugs in the code and adds minor additional functionality
    ///   2. Makes sure everything requested in the spec from a backend standpoint was followed. If not, add the feature. No code should be implemented later. Everything should be written now.
    ///   3. ONLY writes the code. No commentary.
    /// IMPORTANT: The following libraries are already installed. Does not use ANY libraries other than what was provided in the template
    ///   reqwest, serde, serde_json, tokio, actix-web, async-trait
    /// OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code.
    println!(OUTPUT)
}

#[function_to_string]
pub fn print_fixed_code(_broken_code_with_bugs: &str) {
    /// INPUT: Takes in Rust BROKEN_CODE and the ERROR_BUGS found
    /// FUNCTION: Removes bugs from code
    /// IMPORTANT: Only prints out the new and improved code. No commentary or anything else
    println!(OUTPUT)
}

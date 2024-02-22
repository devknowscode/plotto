use super::basic_agent::AgentState;

pub trait BasicTrait {
    fn new(objective: String, position: String) -> Self;
    fn update_state(&mut self, new_state: AgentState);
}

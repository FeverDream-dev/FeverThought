pub struct Planner;
impl Planner {
    pub fn system_prompt() -> &'static str {
        crate::prompts::PLANNER
    }
}

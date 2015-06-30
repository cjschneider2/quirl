use vm_state::VMState;

/// Stores the string representation of the current program as a string slice.
pub struct ProgOutput {
    status : Option<String>,
    std_out: String,
}
pub struct ProgState {
    program : String,
    pp : usize,
}
impl ProgState {
    pub fn new (input: String) -> ProgState {
        ProgState {
            program : input,
            pp : 0,
        }
    }
    pub fn run (&self, vm: &mut VMState) -> ProgOutput {
        let mut std_out = "".to_string();
        let mut output = ProgOutput {
            status: None,
            std_out: std_out,
        };
        output
    }
}

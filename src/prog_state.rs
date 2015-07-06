use vm_state::VMState;

/// Stores the string representation of the current program as a string slice.
#[derive(Debug)]
pub struct ProgOutput {
    pub status : Option<String>,
    pub std_out: Vec<u8>,
}
pub struct ProgState {
    program : Vec<u8>,
    pp : usize,
    len: usize,
}
impl ProgState {
    pub fn new (input: String) -> ProgState {
        let mut ret_val = ProgState {
            program : input,
            pp : 0,
            len: 0,
        };
        ret_val.set_len();
        ret_val
    }
    fn set_len(&mut self) {
        self.len = self.program.len();
    }

    fn inc_pp(&mut self) {
        self.pp += 1;
    }

    pub fn get_pp(&self) -> usize {
        self.pp
    }

    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_next_char(&self) -> Option<&u8> {
        self.inc_pp;
        self.program.get(self.pp)
    }
}

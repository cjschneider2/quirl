//! prog_state: stores the program test/bytecode and current state.

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
    pub fn new (input: Vec<u8>) -> ProgState {
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

    pub fn inc_pp(&mut self) {
        self.pp += 1;
    }

    pub fn get_pp(&self) -> usize {
        self.pp
    }

    pub fn set_pp(&mut self, ptr: u16) {
        self.pp = ptr as usize;
    }

    #[allow(dead_code)]
    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_val(&mut self) -> Option<u8> {
        let val = self.program.get(self.pp);
        if val.is_some() {
            Some(*val.unwrap())
        } else {
            None
        }
    }
}

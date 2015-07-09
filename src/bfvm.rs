use std::collections::HashMap;
use vm_state::VMState;
use prog_state::{ProgState, ProgOutput};

/// The Brainfuck VM
pub struct VM {
    vms : VMState,
    prg : ProgState,
    jmps : HashMap<u16, u16>,
}
impl VM {
    /// parses the imput string, only keeping valid characters.
    /// Also generates the jump table used for the '[' and ']' characters.
    fn parse(input: &str) -> Option<(Vec<u8>,HashMap<u16,u16>)> {
        let mut out_str : Vec<u8> = Vec::new();
        let mut jump_tbl: HashMap<u16,u16> = HashMap::new();
        let mut jmps = Vec::new(); // used to store the intermediate locations
                                   // for the jumps.
        for chr in input.chars() {
            match chr {
                '+' => out_str.push(chr as u8),
                '-' => out_str.push(chr as u8),
                '>' => out_str.push(chr as u8),
                '<' => out_str.push(chr as u8),
                '.' => out_str.push(chr as u8),
                ',' => out_str.push(chr as u8),
                '[' => {
                        out_str.push(chr as u8);
                        jmps.push(out_str.len()-1);
                       },
                ']' => {
                        out_str.push(chr as u8);
                        //TODO: Error recovery
                        let jmp = jmps.pop().unwrap() as u16;
                        jump_tbl.insert(jmp, (out_str.len()-1) as u16 );
                       },
                _ => (),
            }
        }
        Some((out_str,jump_tbl))
    }
    /// runs the VM with the specified input string.
    /// Returns and output object containing the final runtime status of
    /// the program as well as any collected output.
    pub fn run_with_input (input: &str) -> ProgOutput {
        let (prg_str, jmp_tbl) = VM::parse(input).unwrap();
        let mut bf_prog = VM {
            vms: VMState::new(),
            prg: ProgState::new( Vec::<u8>::from(&*prg_str) ),
            jmps: jmp_tbl,
        };
        let output = bf_prog.run();
        output
    }
    fn run (&mut self) -> ProgOutput {
        let mut std_out : Vec<u8> = Vec::new();
        let mut output = ProgOutput {
            status: None,
            std_out: std_out,
        };
        let mut running = true;
        let mut jump_back = 0u16;
        while(running){
            // Stop if we can't get the next char of the program.
            // This probably means we're at the end.
            let next_char = self.prg.get_val();
            // We'll increment our program pointer regardless
            self.prg.inc_pp();
            // And see if there is an instruction to execute
            if next_char.is_some() {
                // Execute the current instruction
                match next_char.unwrap() as char {
                    '+' => self.vms.inc_val(),
                    '-' => self.vms.dec_val(),
                    '>' => self.vms.inc_ptr(),
                    '<' => self.vms.dec_ptr(),
                    ',' => self.vms.load(0),
                    '.' => output.std_out.push(self.vms.store()),
                    '[' => { // Jumps to matching bracket if val at ptr is 0
                            if self.vms.store() == 0 {
                                println!("jump [");
                            }
                            ()
                           },
                    ']' => { // Jumps to matching bracket if val at ptr is non-0
                            if self.vms.store() != 0 {
                                println!("jump ]");
                            }
                            ()
                           },
                    _ => (),
                }
            } else {
                running = false;
            }
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::VM;
    #[test]
    fn test_parse_invalid_symbols() {
        let in_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
            "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let out_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-][>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let o_res : Vec<u8> = Vec::from(out_str);
        let (res,jmp_tbl) = VM::parse(in_str).unwrap();
        assert_eq!(res, o_res);
    }
    #[test]
    fn test_run_with_input() {
        let in_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
            "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let out_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-][>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let o_res : Vec<u8> = Vec::from(out_str);
        let output = VM::run_with_input(in_str);
        println!("{:?}", output);
        assert!(false);
    }
}

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
    fn parse(input: &str) -> Result<(Vec<u8>,HashMap<u16,u16>), &'static str> {
        let mut out_str : Vec<u8> = Vec::new();
        let mut jump_tbl: HashMap<u16,u16> = HashMap::new();
        // "jmps" is used to store the intermediate idx locations used for the jumps.
        let mut jmps = Vec::new();
        let mut match_err = false;
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
                        match jmps.pop() {
                            Some(jmp) => {
                                jump_tbl.insert(jmp as u16, (out_str.len()-1) as u16);
                            },
                            None => { match_err = true; },
                        };
                       },
                _ => (),
            }
        }
        if match_err {
            Err("Unmatched ]")
        } else if jmps.len() > 0 {
            Err("Unmatched [")
        } else {
            Ok((out_str,jump_tbl))
        }
    }
    /// runs the VM with the specified input string.
    /// Returns and output object containing the final runtime status of
    /// the program as well as any collected output.
    pub fn run_with_input (input: &str) -> ProgOutput {
        match VM::parse(input) {
            Ok((prg_str, jump_tbl)) => {
                let mut bf_prog = VM {
                    vms: VMState::new(),
                    prg: ProgState::new( Vec::<u8>::from(&*prg_str) ),
                    jmps: jump_tbl,
                };
                let output = bf_prog.run();
                output
            },
            Err(e_str) => {
                ProgOutput {
                    status: Some(String::from(e_str)),
                    std_out: Vec::<u8>::new(),
                }
            }
        }
    }

    fn run (&mut self) -> ProgOutput {
        let std_out : Vec<u8> = Vec::new();
        let mut output = ProgOutput {
            status: None,
            std_out: std_out,
        };
        let mut running = true;
        while running {
            // Get the program instruction
            let prog_inst = self.prg.get_val();
            // And see if there is an instruction to execute
            if prog_inst.is_some() {
                // Execute the current instruction
                match prog_inst.unwrap() as char {
                    '+' => self.vms.inc_val(),
                    '-' => self.vms.dec_val(),
                    '>' => self.vms.inc_ptr(),
                    '<' => self.vms.dec_ptr(),
                    ',' => self.vms.load(0),
                    '.' => output.std_out.push(self.vms.store()),
                    '[' => {
                            // Set the return pointer for the jump
                            // Jumps to matching bracket if val at ptr is 0
                            if self.vms.store() == 0 {
                                // Find the jump point in the program pointer
                                let jump_back = self.prg.get_pp() as u16;
                                let jmp = self.jmps.get(&jump_back).unwrap();
                                // Jump to the given pointer location
                                self.prg.set_pp(*jmp);
                            }
                           },
                    ']' => {
                            // Jumps to matching bracket if val at ptr is non-0
                            if self.vms.store() != 0 {
                                // Find the correct return pointer
                                let mut ret_ptr = 0;
                                let pp = self.prg.get_pp();
                                for (key, val) in self.jmps.iter() {
                                    if *val == pp as u16 { ret_ptr = *key; break; }
                                }
                                // Set the program pointer back to the orig. val
                                self.prg.set_pp(ret_ptr);
                            }
                           },
                    _ => (),
                }
            } else {
                running = false;
            }
            // We'll increment our program pointer regardless
            self.prg.inc_pp();
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::VM;
    use prog_state::ProgOutput;
    #[test]
    fn test_parse_invalid_symbols() {
        let in_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
            "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let out_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-][>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let o_res : Vec<u8> = Vec::from(out_str);
        let (res,_jmp_tbl) = VM::parse(in_str).unwrap();
        assert_eq!(res, o_res);
    }
    #[test]
    fn test_run_with_input() {
        let in_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
            "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let expected_output = ProgOutput { status: None, std_out: vec!(72,10) };
        let output = VM::run_with_input(in_str);
        println!("{:?}", output);
        assert_eq!(output.std_out, expected_output.std_out);
    }
}

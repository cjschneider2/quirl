use std::collections::HashMap;
use vm_state::VMState;
use prog_state::{ProgState, ProgOutput};

/// The Brainfuck VM
pub struct VM {
    vms : VMState,
    prg : ProgState,
    jmps : HashMap<i16, i16>,
}
impl VM {
    /// parses the imput string, only keeping valid characters.
    /// Also generates the jump table used for the '[' and ']' characters.
    fn parse(input: &str) -> Option<(String,HashMap<i16,i16>)> {
        let mut out_str = String::new();
        let mut jump_tbl: HashMap<i16,i16> = HashMap::new();
        let mut jmps = Vec::new(); // used to store the intermediate locations
                                   // for the jumps.
        for chr in input.chars() {
            match chr {
                '+' => out_str.push(chr),
                '-' => out_str.push(chr),
                '>' => out_str.push(chr),
                '<' => out_str.push(chr),
                '.' => out_str.push(chr),
                ',' => out_str.push(chr),
                '[' => {
                        out_str.push(chr);
                        jmps.push(out_str.len()-1);
                       },
                ']' => {
                        out_str.push(chr);
                        //TODO: Error recovery
                        let jmp = jmps.pop().unwrap() as i16;
                        jump_tbl.insert(jmp, (out_str.len()-1) as i16 );
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
            prg: ProgState::new( prg_str.to_string() ),
            jmps: jmp_tbl,
        };
        let output = bf_prog.run();
        output
    }
    fn run (&mut self) -> ProgOutput {
        let mut std_out = "".to_string();
        let mut output = ProgOutput {
            status: None,
            std_out: std_out,
        };
        let mut running = true;
        while(running){
            // Execute the current instruction
            match self.prg.get_next() {
                '+' => self.vms.inc_val(),
                '-' => self.vms.dec_val(),
                '>' => self.vms.inc_ptr(),
                '<' => self.vms.dec_ptr(),
                '.' => (),
                ',' => (),
                '[' => {
                        ();
                       },
                ']' => {
                        ();
                       },
                _ => (),
            }
            // Inc. program pointer if we have more program to execute
            if self.prg.get_pp() < self.prg.get_len() {
                self.prg.inc_pp();
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
        let o_res = out_str.to_string();
        let (res,jmp_tbl) = VM::parse(in_str).unwrap();
        assert_eq!(res, o_res);
    }
    #[test]
    fn test_run_with_input() {
        let in_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
            "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let out_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-][>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let o_res = out_str.to_string();
        let output = VM::run_with_input(in_str);
        println!("{:?}", output);
        assert!(false);
    }
}

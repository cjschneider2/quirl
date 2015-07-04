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
    fn tokenize (input: &str) -> Option<(String,HashMap<i16,i16>)> {
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
    pub fn run_with_input (input: &str) -> ProgOutput {
        let (prg_str, jmp_tbl) = VM::tokenize(input).unwrap();
        let mut bf_prog = VM {
            vms: VMState::new(),
            prg: ProgState::new( prg_str.to_string() ),
            jmps: jmp_tbl,
        };
        let output = bf_prog.prg.run(&mut bf_prog.vms);
        output
    }
}

#[cfg(test)]
mod test {
    use super::VM;
    #[test]
    fn tok_valid_symbol_test () {
        let in_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
            "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let out_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-][>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let o_res = out_str.to_string();
        let (res,jmp_tbl) = VM::tokenize(in_str).unwrap();
        assert_eq!(res, o_res);
    }
}

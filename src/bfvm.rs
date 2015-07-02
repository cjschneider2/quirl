use vm_state::VMState;
use prog_state::{ProgState, ProgOutput};

/// The Brainfuck VM
pub struct VM {
    vms : VMState,
    prg : ProgState,
}
impl VM {
    pub fn run_with_input (input: &str) -> ProgOutput {
        let mut bf_prog = VM {
            vms: VMState::new(),
            prg: ProgState::new( input.to_string() ),
        };
        let output = bf_prog.prg.run(&mut bf_prog.vms);
        output
    }
    fn tokenize_input (input: &str) -> Option<String> {
        let mut tmp_str = String::new();
        for chr in input.chars() {
            match chr {
                '+' => tmp_str.push(chr),
                '-' => tmp_str.push(chr),
                '>' => tmp_str.push(chr),
                '<' => tmp_str.push(chr),
                '[' => tmp_str.push(chr),
                ']' => tmp_str.push(chr),
                '.' => tmp_str.push(chr),
                ',' => tmp_str.push(chr),
                _ => (),
            }
        }
        Some(tmp_str)
    }
}

mod test {
    use super::VM;
    #[test]
    fn tok_valid_symbol_test () {
        let in_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
            "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let out_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-][>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let o_res = out_str.to_string();
        let res = VM::tokenize_input(in_str).unwrap();
        assert_eq!(res, o_res);
    }
}

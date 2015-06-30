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
        Some("string".to_string())
    }
}

/*
mod test {
    use ::VM::tokenize_input;
    #[test]
    fn tok_valid_symbol_test () {
        let in_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
            "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let out_str = r#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-][>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let res = VM::tokenize_input(in_str);
        assert_eq!(res,out_str);
    }
}
*/

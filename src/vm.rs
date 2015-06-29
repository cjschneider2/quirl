/// Brainfuck is represented by an array with 30,000 cells initialized to zero
/// and a data pointer pointing at the current cell.
///
/// There are eight commands:
/// + : Increments the value at the current cell by one.
/// - : Decrements the value at the current cell by one.
/// > : Moves the data pointer to the next cell (cell on the right).
/// < : Moves the data pointer to the previous cell (cell on the left).
/// . : Prints the ASCII value at the current cell (i.e. 65 = 'A').
/// , : Reads a single input character into the current cell.
/// [ : If the value at the current cell is zero, skips to the corresponding ] .
///     Otherwise, move to the next instruction.
/// ] : If the value at the current cell is zero, move to the next instruction.
///     Otherwise, move backwards in the instructions to the corresponding [ .
struct VMState {
    stack : [u16; 30000],
    p : usize,
}
impl VMState {
    fn new () -> VMState {
        VMState {
            stack : [0; 30000],
            p : 0
        }
    }
    /// + : Increments the value at the current cell by one.
    fn inc_val (&mut self) {
        self.stack[self.p] += 1;
    }
    /// - : Decrements the value at the current cell by one.
    fn dec_val (&mut self) {
        self.stack[self.p] -= 1;
    }
    /// > : Moves the data pointer to the next cell (cell on the right).
    fn inc_ptr (&mut self) {
        self.p += 1;
    }
    /// < : Moves the data pointer to the previous cell (cell on the left).
    fn dec_ptr (&mut self) {
        self.p -= 1;
    }
    /// . : Prints the ASCII value at the current cell (i.e. 65 = 'A').
    fn std_out (&mut self) {
        ()
    }
    /// , : Reads a single input character into the current cell.
    fn std_in (&mut self) {
        ()
    }
    /// [ : If the value at the current cell is zero, skips to the corresponding ] .
    ///     Otherwise, move to the next instruction.
    fn while_lhs (&mut self) {
        ()
    }
    /// ] : If the value at the current cell is zero, move to the next instruction.
    ///     Otherwise, move backwards in the instructions to the corresponding [ .
    fn while_rhs (&mut self) {
        ()
    }
}

/// Stores the string representation of the current program as a string slice.
pub struct ProgOutput {
    status : Option<String>,
    std_out: String,
}
struct ProgState {
    program : String,
    pp : usize,
}
impl ProgState {
    fn new (input: String) -> ProgState {
        ProgState {
            program : input,
            pp : 0,
        }
    }
    fn run (&self, vm: &mut VMState) -> ProgOutput {
        let mut std_out = "".to_string();
        let mut output = ProgOutput {
            status: None,
            std_out: std_out,
        };
        output
    }
}

/// A Brainfuck VM consists of the program state and the VM state.
/// which is exposed thought the BF fn's
pub struct BF {
    vms : VMState,
    prg : ProgState,
}
impl BF {
    pub fn run_with_input (input: &str) -> ProgOutput {
        let mut bf_prog = BF {
            vms: VMState::new(),
            prg: ProgState::new( input.to_string() ),
        };
        let output = bf_prog.prg.run(&mut bf_prog.vms);
        output
    }
}

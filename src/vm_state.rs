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
/// [ : If the value at the current cell is zero, skips to the
///     corresponding ] . Otherwise, move to the next instruction.
/// ] : If the value at the current cell is zero, move to the next instruction.
///     Otherwise, move backwards in the instructions to the corresponding [ .
///
/// Note: the previous descriptions are according to the def'n of the spec.
///       Implementation specific descriptions are with the corresponding fns.
pub struct VMState {
    stack : [u8; 30000],
    p : usize,
}
impl VMState {
    pub fn new () -> VMState {
        VMState {
            stack : [0; 30000],
            p : 0
        }
    }
    /// '+' : Increments the value at the current cell by one.
    fn inc_val (&mut self) {
        self.stack[self.p] = self.stack[self.p].wrapping_add(1);
    }
    /// '-' : Decrements the value at the current cell by one.
    fn dec_val (&mut self) {
        self.stack[self.p] = self.stack[self.p].wrapping_sub(1);
    }
    /// '>' : Moves the data pointer to the next cell (cell on the right).
    fn inc_ptr (&mut self) {
        self.p += 1;
    }
    /// '<' : Moves the data pointer to the previous cell (cell on the left).
    fn dec_ptr (&mut self) {
        self.p -= 1;
    }
    /// '.' : Returns the ASCII value at the current cell (i.e. 65 = 'A').
    fn std_out (&mut self) -> u8 {
        return (self.stack[self.p]);
    }
    /// ',' : Reads a single input character into the current cell.
    fn std_in (&mut self, input:u8) {
        self.stack[self.p] = input;
    }

    // '[' and ']' are purely program state constructs.
    /*
    /// [ : If the value at the current cell is zero, skips to the
    /// corresponding ']'. Otherwise, move to the next instruction.
    fn while_lhs (&mut self) {
        ()
    }
    /// ] : If the value at the current cell is zero, move to the next
    ///     instruction. Otherwise, move backwards in the instructions
    ///     to the corresponding '[' .
    fn while_rhs (&mut self) {
        ()
    }
    */
}

mod test {
    use super::*;
    #[test]
    fn vm_test_in_out () {
        let mut vm = VMState::new();
        let in_val:u8 = 123;
        vm.std_in(in_val);
        let out_val = vm.std_out();
        assert_eq!(in_val, out_val);
    }
    #[test]
    fn vm_test_inc_val () {
        let mut vm = VMState::new();
        vm.inc_val();
        assert_eq!(vm.std_out(), 1);
    }
    #[test]
    fn vm_test_dec_val () {
        let mut vm = VMState::new();
        vm.stack[vm.p] = 2;
        vm.dec_val();
        assert_eq!(vm.std_out(), 1);
    }
    #[test]
    fn vm_test_underflow_val () {
        let mut vm = VMState::new();
        vm.dec_val();
        assert_eq!(vm.std_out(), 255);
    }
    #[test]
    fn vm_test_overflow_val () {
        let mut vm = VMState::new();
        vm.stack[vm.p] = 255;
        vm.inc_val();
        assert_eq!(vm.std_out(), 0);
    }
    #[test]
    fn vm_test_inc_ptr () {
        let mut vm = VMState::new();
        vm.stack[0] = 0;
        vm.stack[1] = 1;
        vm.stack[2] = 2;
        vm.stack[3] = 3;
        vm.inc_ptr();
        assert_eq!(vm.std_out(), 1);
        vm.inc_ptr();
        assert_eq!(vm.std_out(), 2);
        vm.inc_ptr();
        assert_eq!(vm.std_out(), 3);
    }
    #[test]
    fn vm_test_dec_ptr () {
        let mut vm = VMState::new();
        vm.stack[0] = 0;
        vm.stack[1] = 1;
        vm.stack[2] = 2;
        vm.stack[3] = 3;
        vm.p = 3;
        vm.dec_ptr();
        assert_eq!(vm.std_out(), 2);
        vm.dec_ptr();
        assert_eq!(vm.std_out(), 1);
        vm.dec_ptr();
        assert_eq!(vm.std_out(), 0);
    }
}

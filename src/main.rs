//! Quirl is a brainfuck interpreter / REPL

mod vm_state;
mod prog_state;
mod bfvm;

#[cfg(not(test))]
fn main() {
    // The welcome text in beautiful bf bytecode :P
    let welcome_prog = "+[--->++<]>+.++[->++++<]>+.+++++++.----
        -----.++++++++++++.--.--------.--[--->+<]>-.---[->++++<
        ]>.-----.[--->+<]>-----.---[->++++<]>.------------.---.
        --[--->+<]>-.[-->+++++<]>+.---[-->+++<]>.------------.+
        ++++++++.------.[++>---<]>--.+[->++<]>.---[----->+<]>-.
        +++[->+++<]>++.++++++++.+++++.--------.-[--->+<]>--.+[-
        >+++<]>+.++++++++.-[++>---<]>+.++++[->++<]>+.+[--->+<]>
        .++++++.+++[->+++<]>.+++++++++++++.--.++.-------------.
        [--->+<]>---.+++[->+++<]>.+++++++++++++.[--->+<]>-----.";
    let res = bfvm::VM::run_with_input(welcome_prog);
    let out_str = std::str::from_utf8(&res.std_out);
    match out_str {
        Ok(string) => println!("{}", string),
        Err(error) => println!("{}", error),
    }
    println!("");
    // The actual REPL starts here
    loop {
        // Read
        print!("> ");
        let _  = io::stdout().flush();
        let mut input = String::new();
        let in_len = io::stdin()
                        .read_line(&mut input)
                        .ok()
                        // This is a little bit of a cop-out until I
                        // figure out what to do with invalid input
                        .unwrap_or(0);
        if in_len > 0 {
            // Evaluate
            let result = bfvm::VM::run_with_input(&input);
            // Print
            let out_str = std::str::from_utf8(&result.std_out);
            match out_str {
                Ok(string) => println!("{}", string),
                Err(error) => println!("{}", error),
            }
        } else {
            println!("Unhelpful error message about bad input.")
        }
        // Loop
    }
}


#[cfg(test)]
mod test {
    use bfvm;
    use prog_state::ProgOutput;
    use std;

    #[test]
    fn output_2_program() {
        let prog_string = "++.";
        let expected_output = ProgOutput { status: None, std_out: vec!(2) };
        let vm_output = bfvm::VM::run_with_input(prog_string);
        assert_eq!(vm_output.std_out, expected_output.std_out);
    }
    #[test]
    fn hello_world() {
        let prog_str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let res = bfvm::VM::run_with_input(prog_str);
        let out_str = std::str::from_utf8(&res.std_out);
        let res = match out_str {
            Ok(string) => string,
            Err(error) => {println!("{}", error); ""},
        };
        assert_eq!(res, "Hello World!\n");
    }


    /// The following tests are obtained from :
    ///    Daniel B Cristofani (cristofdathevanetdotcom)
    ///    http://www.hevanet.com/cristofd/brainfuck/
    ///    Here are some little programs for testing brainfuck implementations.


    /// >,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.
    /// This is for testing i/o; give it a return followed by an EOF. (Try it both
    /// with file input--a file consisting only of one blank line--and with
    /// keyboard input, i.e. hit return and then ctrl-d (Unix) or ctrl-z
    /// (Windows).)
    /// It should give two lines of output; the two lines should be identical, and
    /// should be lined up one over the other. If that doesn't happen, ten is not
    /// coming through as newline on output.
    /// The content of the lines tells how input is being processed; each line
    /// should be two uppercase letters.
    /// Anything with O in it means newline is not coming through as ten on input.
    /// LK means newline input is working fine, and EOF leaves the cell unchanged
    /// (which I recommend).
    /// LB means newline input is working fine, and EOF translates as 0.
    /// LA means newline input is working fine, and EOF translates as -1.
    /// Anything else is fairly unexpected.
    #[test]
    fn test_new_line() {
        let prog_str = ">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.";
        let res = bfvm::VM::run_with_input(prog_str);
        let out_str = std::str::from_utf8(&res.std_out);
        let res = match out_str {
            Ok(string) => {println!("{}", string); string},
            Err(error) => {println!("{}", error); ""},
        };
        assert_eq!(res, "BB\nBB\n");
    }

    /// ++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]
    /// +++++[>+++++++<<++>-]>.<<.
    /// Goes to cell 30000 and reports from there with a #. (Verifies that the
    /// array is big enough.)
    #[test]
    fn test_cell_30000() {
        let prog_str = "++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]+++++[>+++++++<<++>-]>.<<.";
        let res = bfvm::VM::run_with_input(prog_str);
        let out_str = std::str::from_utf8(&res.std_out);
        let res = match out_str {
            Ok(string) => {println!("{}", string); string},
            Err(error) => {println!("{}", error); ""},
        };
        assert_eq!(res, "#\n");
    }

    /*
    These next two test the array bounds checking. Bounds checking is not
    essential, and in a high-level implementation it is likely to introduce
    extra overhead. In a low-level implementation you can get bounds checking
    for free by using the OS's own memory protections; this is the best
    solution, which may require making the array size a multiple of the page
    size.
    Anyway. These two programs measure the "real" size of the array, in some
    sense, in cells left and right of the initial cell respectively. They
    output the result in unary; the easiest thing is to direct them to a file
    and measure its size, or (on Unix) pipe the output to wc. If bounds
    checking is present and working, the left should measure 0 and the right
    should be the array size minus one.
    +[<+++++++++++++++++++++++++++++++++.]

    +[>+++++++++++++++++++++++++++++++++.]
    */

    /// []++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
    /// "A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>.
    /// Tests for several obscure problems. Should output an H.
    #[test]
    fn test_obscure_problems() {
        let prog_str = r#""[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]"A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
        let res = bfvm::VM::run_with_input(prog_str);
        let out_str = std::str::from_utf8(&res.std_out);
        let res = match out_str {
            Ok(string) => {println!("{}", string); string},
            Err(error) => {println!("{}", error); ""},
        };
        assert_eq!(res, "H\n");
    }

    /// +++++[>+++++++>++<<-]>.>.[
    /// Should ideally give error message "unmatched [" or the like, and not give
    /// any output. Not essential.
    #[test]
    fn test_unmatched_l_bracket() {
        let prog_str = r#"+++++[>+++++++>++<<-]>.>.["#;
        let res = bfvm::VM::run_with_input(prog_str);
        println!("{:?}", res.status);
        assert!(res.status.is_some());
    }

    /// +++++[>+++++++>++<<-]>.>.][
    /// Should ideally give error message "unmatched ]" or the like, and not give
    /// any output. Not essential.
    #[test]
    fn test_unmatched_r_bracket() {
        let prog_str = r#"+++++[>+++++++>++<<-]>.>.]["#;
        let res = bfvm::VM::run_with_input(prog_str);
        println!("{:?}", res.status);
        assert!(res.status.is_some());
    }
}

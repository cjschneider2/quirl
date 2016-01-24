## Quirl - A Brainfuck interpreter in Rust

This program started as an experiment implementing a simple VM and I settled on Brainfuck.

Quirl runs a simple REPL where input text is run through the BF interpreter and the
resulting output is displayed.

### Implementation detail
As some BF programs are implementation dependent, Some details of this one are given here.

* The VM implements a 30 000 cell machine. 
* Each cell is limited to u8 size, such that any valid ASCII caracter may be represented.
* Wrapping of the cell pointer is not allowed, and considered an error.
* Wrapping of cell values are allowed.

### Contributing
Suggestions and/or pull requests are welcome!

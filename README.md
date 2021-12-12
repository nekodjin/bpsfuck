# BPSFuck

BPSFuck is a BrainFuck interpreter submitted for the [2021 #BPSCoders Student
Contest][bpscsc2021]. It was submitted for contest No. 6.

BrainFuck is an esoteric programming language created in the early 90's. It
consists of a set of 8 "commands" which operate on a (theoretically) infinite
tape of byte-sized cells, each holding a value from 0 to 255. The commands are
as follows:

| Command | Effect                                                |
| ------- | ----------------------------------------------------- |
| `+`     | Increment the current cell                            |
| `-`     | Decrement the current cell                            |
| `>`     | Shift the tape one cell to the right                  |
| `<`     | Shift the tape one cell to the left                   |
| `.`     | Output the value of the current cell                  |
| `,`     | Take a byte of input and store it in the current cell |

This list contains only 6 commands, as the remaining two come in delimiting
pairs. They are `[` and `]`, and they must come in matching pairs - somewhat
like parentheses. Together, they form a loop which repeats the enclosed
commands for as long as the current cell (that is, the cell in focus at the
beginning of each repition) is nonzero. The above table could be continued as
follows:

| Command | Effect                                                              |
| ------- | ------------------------------------------------------------------- |
| `[`     | Jump to command after corresponding `]` iff current cell is zero    |
| `]`     | Jump to command after corresponding `[` iff current cell is nonzero |

The definition of either of these commands can be replaced with "unconditional
jump to the corresponding `[`/`]`" without changing the behavior of the system,
but not both.

You can read more about BrainFuck on the [Esolang Wiki][esowikibf].

To run this interpreter, you will need the Rust toolchain installed. You can
find downloads and installation instructions for supported platforms on the
[Rust website][rust-init].

To run the interpreter, invoke the command `cargo run -- source.bf` in your
terminal of choice, where `source.bf` is the path to the file containing your
BrainFuck source code. 

Alternatively, you can install the interpreter with by invoking the command
`cargo install bpsfuck`, and then run the interpreter with `bpsfuck source.bf`.

#### TODO:
- [ ] Support arbitrary chars as no-op

This project is licensed under the BSD 3-Clause license. 

[bpscsc2021]: <https://sites.google.com/bostonpublicschools.org/bpscoders/2021-student-contest> "2021 #BPSCoders Student Contest"
[rust-init]:  <https://rust-lang.org/tools/install> "Install Rust"
[esowikibf]:  <https://esolangs.org/wiki/Brainfuck> "BrainFuck on Esolangs Wiki"


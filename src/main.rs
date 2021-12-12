use std::{fs, env};
use std::io::{Read, Write, stdin, stdout};
use std::collections::HashMap;
use chumsky::prelude::*;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    parser().parse(src.trim()).unwrap().run();
}

#[derive(Clone)]
enum Instr {
    CInc,
    CDec,
    PInc,
    PDec,
    PChr,
    GChr,
    Loop(Instrs),
}

#[derive(Clone)]
struct Instrs(Vec<Instr>);

impl Instrs {
    pub fn run(&self) {
        let mut tape = HashMap::new();
        tape.insert(0, 0);
        self.exec(&mut 0, &mut tape);
    }

    fn exec(&self, i: &mut usize, tape: &mut HashMap<usize, u8>) {
        use Instr::*;

        for instr in self.0.iter() {
            match instr {
                CInc => {
                    tape.insert(*i, tape[i].wrapping_add(1));
                }
                CDec => {
                    tape.insert(*i, tape[i].wrapping_sub(1));
                }
                PInc => {
                    *i = i.wrapping_add(1);
                    if !tape.contains_key(i) {
                        tape.insert(*i, 0);
                    }
                }
                PDec => {
                    *i = i.wrapping_sub(1);
                    if !tape.contains_key(i) {
                        tape.insert(*i, 0);
                    }
                }
                PChr => {
                    print!("{}", tape[i] as char);
                    stdout().flush().unwrap();
                }
                GChr => {
                    tape.insert(*i, stdin().lock().bytes().next().unwrap().unwrap());
                }
                Loop(instrs) => {
                    while tape[i] != 0 {
                        instrs.exec(i, tape);
                    }
                }
            }
        }
    }
}

fn parser() -> impl Parser<char, Instrs, Error = Simple<char>> {
    use Instr::*;

    recursive(|instr| instr
        .repeated()
        .delimited_by('[', ']')
        .map(Instrs)
        .map(Loop)
        .or(just('+').to(CInc))
        .or(just('-').to(CDec))
        .or(just('>').to(PInc))
        .or(just('<').to(PDec))
        .or(just('.').to(PChr))
        .or(just(',').to(GChr))
    )
        .repeated()
        .map(Instrs)
        .then_ignore(end())
}


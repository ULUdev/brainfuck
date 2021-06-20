// some general help stuff
pub mod help {
    pub fn match_closing_paren(string: String, open_pos: usize) -> Option<usize> {
        let chars: Vec<char> = string.chars().collect();
        let mut counter: i32 = 1;
        let mut pos: usize = open_pos;
        while counter > 0 {
            pos += 1;
            match chars[pos] {
                '[' => counter += 1,
                ']' => counter -= 1,
                _ => {}
            }
        }
        Some(pos)
    }
}

pub mod interpreter {
    pub use crate::brainfuck::help::match_closing_paren;
    pub use std::io::{self, Write};

    #[derive(Debug, Clone)]
    pub struct ProgramState {
        mem: Vec<u8>,
        ptr: usize,
        inbuf: Vec<char>,
    }

    impl ProgramState {
        pub fn new(mem: Vec<u8>, ptr: usize) -> ProgramState {
            ProgramState { mem, ptr, inbuf: Vec::new() }
        }

        pub fn incr_ptr(&mut self) {
            if self.ptr == self.mem.len() - 1 {
                self.ptr = 0;
            } else {
                self.ptr += 1;
            }
        }

        pub fn decr_ptr(&mut self) {
            if self.ptr == 0 {
                self.ptr = self.mem.len() - 1;
            } else {
                self.ptr -= 1;
            }
        }

        pub fn incr_ptr_val(&mut self) {
            if self.mem[self.ptr] == 255 {
                self.mem[self.ptr] = 0;
            } else {
                self.mem[self.ptr] += 1;
            }
        }

        pub fn decr_ptr_val(&mut self) {
            if self.mem[self.ptr] == 0 {
                self.mem[self.ptr] = 255;
            } else {
                self.mem[self.ptr] -= 1;
            }
        }

        pub fn print_from_ptr(&self) {
            let val: String = String::from_utf8(vec![self.mem[self.ptr]]).unwrap();
            print!("{}", val);
            io::stdout().flush().unwrap();
        }

        pub fn read_to_ptr(&mut self) {
            if self.inbuf.len() == 0 {
                let mut input: String = String::new();
                let _res = io::stdin().read_line(&mut input);
                let in_chars: Vec<char> = input.chars().collect();
                if in_chars.len() == 0 {
                    return;
                }
                let input_as_u8: u8 = in_chars[0] as u8;
                if in_chars.len() > 1 {
                    let push_chars: Vec<char> = in_chars[1..].to_vec();
                    for ch in push_chars {
                        self.inbuf.push(ch);
                    }
                }
                self.mem[self.ptr] = input_as_u8;
            }
            else {
                self.mem[self.ptr] = self.inbuf[0] as u8;
                self.inbuf.remove(0);
            }
        }

        pub fn get_current(&self) -> u8 {
            self.mem[self.ptr]
        }
    }

    pub fn execute_statements(statements: &Vec<char>, inital_state: &ProgramState) -> ProgramState {
        let mut new_state: ProgramState = inital_state.clone();
        let mut new_statements: Vec<char> = statements.clone();
        while new_statements.len() != 0 {
            let stat: char = new_statements[0];
            match stat {
                '+' => {
                    new_state.incr_ptr_val();
                }
                '-' => {
                    new_state.decr_ptr_val();
                }
                '>' => {
                    new_state.incr_ptr();
                }
                '<' => {
                    new_state.decr_ptr();
                }
                '.' => {
                    new_state.print_from_ptr();
                }
                ',' => {
                    new_state.read_to_ptr();
                }
                '[' => {
                    let statement_string: String = new_statements.clone().into_iter().collect();
                    let closing: usize = match match_closing_paren(statement_string, 0) {
                        Some(n) => n,
                        None => {
                            println!("no closing bracket found!");
                            return ProgramState::new(vec![0; 512], 0);
                        }
                    };
                    if closing == 0 {
                        println!("no closing bracket found!");
                        return ProgramState::new(vec![0; 512], 0);
                    }
                    while new_state.get_current() != 0 {
                        new_state = execute_statements(&new_statements[1..closing].to_vec(), &new_state);
                    }
                    let mut statements_left: Vec<char> = Vec::new();
                    if closing + 1 == new_statements.len() {
                        statements_left = new_statements[closing..].to_vec();
                    } else if closing + 1 < new_statements.len() {
                        statements_left = new_statements[closing+1..].to_vec();
                    }
                    //let statements_left: Vec<char> = new_statements[closing+1..].to_vec();
                    new_statements = vec![new_statements[0]];
                    new_statements.extend(statements_left);
                }
                ']' => {
                    break;
                }
                _ => {}
            }
            new_statements.remove(0);
        }

        new_state
    }
}

pub mod cmd {
    pub const HELP: &str = "
        brainfuck [options] <file>
        run brainfuck code

        options:
        -d, --debug: enable debugging output after running the programm
        -h, --help: show this help message
        -v, --version: print the version number
    ";
    pub const VERSION: &str = "brainfuck: v1.0.1";
}
// vim: ts=4 sts=4 sw=4 expandtab

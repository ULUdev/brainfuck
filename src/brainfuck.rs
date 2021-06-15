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

    #[derive(Debug, Clone)]
    pub struct ProgramState {
        mem: Vec<u8>,
        ptr: usize,
    }

    impl ProgramState {
        pub fn new(mem: Vec<u8>, ptr: usize) -> ProgramState {
            ProgramState { mem, ptr }
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
        }

        pub fn read_to_ptr(&mut self) {
            // TODO: add stdin reading to here
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
                    let statements_left: Vec<char> = new_statements[closing..].to_vec();
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
    ";
}
// vim: ts=4 sts=4 sw=4 expandtab

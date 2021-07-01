use std::io::Error;

use std::fs;

mod brainfuck;
use brainfuck::interpreter::*;

fn main() -> Result<(), Error> {
    // debugging interface (just print the ProgramState at the end)
    let mut debug: bool = false;

    // checking the arguments provided
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Ok(());
    }
    for arg in &args[1..].to_vec() {
        match arg.as_str() {
            "-d" | "--debug" => {
                debug = true;
            }
            "-h" | "--help" => {
                println!("{}", brainfuck::cmd::HELP);
            }
            "-v" | "--version" => {
                println!("{}", brainfuck::cmd::VERSION);
            }
            _ => {}
        }
    }

    let mut argclone: Vec<String> = args.clone();
    let last_arg_chars: Vec<char> = argclone.pop().unwrap().chars().collect();
    if last_arg_chars[0] == '-' {
        return Ok(());
    } else {
        let fname: String = args.pop().unwrap();
        let content: String = fs::read_to_string(fname.as_str())?;
        let statements: Vec<char> = content.chars().collect();
        let init_ptr: usize = 0;
        let init_mem: Vec<u8> = vec![0; 512];
        let state: ProgramState = ProgramState::new(init_mem, init_ptr);
        let new_state = match execute_statements(&statements, &state) {
            Ok(n) => n,
            Err(e) => {
                println!("error at runtime: {}", e);
                return Err(Error::new(std::io::ErrorKind::Other, "error at runtime"));
            }
        };
        if debug {
            dbg!(new_state);
        }
        return Ok(());
    }
}

// vim: ts=4 sts=4 sw=4 expandtab

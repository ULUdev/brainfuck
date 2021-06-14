mod brainfuck;
use brainfuck::interpreter::*;
fn main() {
	let init_ptr: usize = 0;
	let init_mem: Vec<u8> = vec![0; 16];
    let state: ProgramState = ProgramState::new(init_mem, init_ptr);
	let new_state = execute_statements(&vec!['+', '>', '-', '>', '+'], &state);
	dbg!(new_state);
}

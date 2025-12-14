use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let instructions: String = fs::read_to_string(&args[1]).expect("Failed to load file");

    let mut cells: Vec<u8> = vec![0; 30000];
    let mut jump_stack: Vec<usize> = Vec::new();
    let mut cell_ptr: usize = 0;
    let mut instr_ptr: usize = 0;

    let operations: Vec<char> = vec!['+', '-', ',', '.', '>', '<', '[', ']'];

    let instr_vec: Vec<char> = instructions.chars().collect();

    while instr_ptr < instr_vec.len() {
        if !operations.contains(&instr_vec[instr_ptr]) {
            instr_ptr += 1;
            continue;
        }

        match instr_vec[instr_ptr] {
            '+' => {
                cells[cell_ptr] += 1;
            }
            '-' => {
                cells[cell_ptr] -= 1;
            }
            '>' => {
                cell_ptr += 1;
            }
            '<' => {
                cell_ptr -= 1;
            }
            '.' => {
                print!("{}", cells[cell_ptr] as char);
            }
            '[' => {
                if cells[cell_ptr] == 0 {
                    instr_ptr = jump_loop(instr_ptr, &instr_vec);
                } else {
                    jump_stack.push(instr_ptr);
                };
            }
            ']' => {
                if cells[cell_ptr] != 0 {
                    instr_ptr = jump_stack[jump_stack.len() - 1];
                } else {
                    jump_stack.pop();
                };
            }
            ',' => {
                let mut input: String = String::new();
                io::stdin().read_line(&mut input).expect("Input Failed");

                cells[cell_ptr] = input.trim().parse::<u8>().unwrap();
            }
            _ => {}
        }

        instr_ptr += 1;
    }
}

fn jump_loop(i: usize, instr: &Vec<char>) -> usize {
    let mut index = i;
    let mut pass: i32 = 0;

    while instr[index] != ']' || pass != 0 {
        if instr[index] == '[' {
            pass += 1;
        }

        if instr[index] == ']' {
            pass -= 1;
        }

        index += 1;
    }

    return index;
}

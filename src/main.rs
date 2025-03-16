use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });


            if !file_contents.is_empty() { 
                tokenize(&file_contents)
            } else {
                println!("EOF  null"); 
            }
            
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(input: &str){
    let mut x : i32 = 1;
    let mut had_error= false;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            '*' => println!("STAR * null"),
            ',' => println!("COMMA , null"),
            '+' => println!("PLUS + null"),
            '.' => println!("DOT . null"),
            '-' => println!("MINUS - null"),
            ';' => println!("SEMICOLON ; null"),
            '\n' => { x += 1 },
            ' ' => {},
            '=' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('=') {
                    println!("EQUAL_EQUAL == null");
                    chars.next();
                }else{
                    println!("EQUAL = null")
                }
            },
            '!' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('='){
                    println!("BANG_EQUAL != null");
                    chars.next();
                }else{
                    println!("BANG ! null")
                }
            }
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", x, c);
                had_error = true;
            }
        }
    }

    println!("EOF  null");
    if had_error {
        process::exit(65);
    }
}

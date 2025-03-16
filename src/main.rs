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
    let mut tokens : Vec<String> = Vec::new();
    let mut x : i32 = 1;
    let mut had_error= false;
    let mut accept_str = false;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push("LEFT_PAREN ( null".to_string()),
            ')' => tokens.push("RIGHT_PAREN ) null".to_string()),
            '{' => tokens.push("LEFT_BRACE { null".to_string()),
            '}' => tokens.push("RIGHT_BRACE } null".to_string()),
            '*' => tokens.push("STAR * null".to_string()),
            ',' => tokens.push("COMMA , null".to_string()),
            '+' => tokens.push("PLUS + null".to_string()),
            '.' => tokens.push("DOT . null".to_string()),
            '-' => tokens.push("MINUS - null".to_string()),
            ';' => tokens.push("SEMICOLON ; null".to_string()),
            '\n' => { x += 1 },
            ' ' => {},
            '\t' => {},
            '=' => {
                if tokens.last() == Some(&"EQUAL = null".to_string()) {
                    tokens.pop();
                    tokens.push("EQUAL_EQUAL == null".to_string());
                } else if tokens.last() == Some(&"BANG ! null".to_string()){
                    tokens.pop();
                    tokens.push("BANG_EQUAL != null".to_string());
                } else if tokens.last() == Some(&"LESS < null".to_string()){
                    tokens.pop();
                    tokens.push("LESS_EQUAL <= null".to_string());
                }else if tokens.last() == Some(&"GREATER > null".to_string()){
                    tokens.pop();
                    tokens.push("GREATER_EQUAL >= null".to_string());
                } else {
                    tokens.push("EQUAL = null".to_string());
                }
            },
            '!' => tokens.push("BANG ! null".to_string()),
            '<' => tokens.push("LESS < null".to_string()),
            '>' => tokens.push("GREATER > null".to_string()),
            '/' => {
                let mut peekable = chars.clone().peekable();
                if peekable.next() == Some('/'){
                    while let Some(ctrim) = chars.next(){
                        if ctrim == '\n'{
                            x += 1;
                            break;
                        }
                    }
                }else{
                    tokens.push("SLASH / null".to_string())
                }
            },
            '"' => {
                accept_str = true;
                let mut literal : String = String::new();
                while let Some(ctrim) = chars.next() {

                    if ctrim == '\n' {
                        accept_str = false;
                        eprintln!("[line {}] Error: Unterminated string.", x);
                        had_error = true;
                        continue;
                    }

                    if ctrim == '"' {
                        accept_str = false;
                        tokens.push(format!("STRING \"{}\" {}", literal, literal));
                        break;
                    }
                    
                    literal.push(ctrim);
                }
            }
            _ => {
                tokens.push("Error".to_string());
                eprintln!("[line {}] Error: Unexpected character: {}", x, c);
                had_error = true;
            }
        }

        if accept_str{
            eprintln!("[line {}] Error: Unterminated string.", x);
            had_error = true;
        }
    }

    for token in &tokens{
        if token == "Error" {
            continue;
        }
        println!("{}",token)
    }
    println!("EOF  null");
    if had_error {
        process::exit(65);
    }
}

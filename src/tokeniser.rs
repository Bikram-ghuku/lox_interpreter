use std::process;

pub fn tokenize(input: &str){
    let mut tokens : Vec<String> = Vec::new();
    let mut x : i32 = 1;
    let mut had_error= false;
    let mut accept_str = false;
    let mut chars = input.chars().peekable();
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
            },
            '0'..='9' => {
                let mut dec_det = false;
                let mut num_literal : String = String::from(c);

                while let Some(c) = chars.peek(){
                    if *c == '.' && !dec_det{
                        num_literal.push(*c);
                        dec_det = true;
                        chars.next();
                    }else if c.is_digit(10){
                        num_literal.push(*c);
                        chars.next();
                    }else{
                        break;
                    }
                }

                if num_literal.ends_with('.') {
                    num_literal.push('0');
                }
                
                if num_literal.ends_with(".0") {
                    tokens.push(format!("NUMBER {} {}", num_literal.replace(".0", ""), num_literal));
                } else if !num_literal.contains('.'){
                    tokens.push(format!("NUMBER {} {}.0", num_literal, num_literal));
                }else{
                    let mut no_trail_zero : String = num_literal.clone();
                    let mut zer_rem: bool = false;
                    while no_trail_zero.ends_with('0') {
                        no_trail_zero.pop();
                        zer_rem = true;
                    }

                    if zer_rem {
                        no_trail_zero.push('0');
                    }
                    tokens.push(format!("NUMBER {} {}", num_literal, no_trail_zero));
                }

            },
            s if s.is_alphabetic() || s == '_' => {
                let mut identifiers: String = String::from(s);
                while let Some(c) = chars.peek() {
                    if c.is_alphanumeric() || *c == '_' {
                        identifiers.push(*c);
                        chars.next();
                    }else{
                        break;
                    }
                }

                tokens.push(format!("IDENTIFIER {} null", identifiers));
            },
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

use crate::tokeniser::tokenize;
use std::process;

pub fn parse(input: &str){
    let (tokens , had_error) = tokenize(input);
    let mut parse : Vec<String> = Vec::new();
    for token in &tokens{
        match token.as_str() {
            st if st.starts_with("TRUE") || st.starts_with("FALSE") || st.starts_with("NIL")=> {
                let chs: Vec<&str> = st.split(" ").collect();
                parse.push(format!("{}", chs[1]));
            }

            st if st.starts_with("NUMBER") => {
                let chs: Vec<&str> = st.split(" ").collect();
                parse.push(format!("{}", chs[2]));
            }
            st if st.starts_with("STRING") => {
                let chs: Vec<&str> = st.split("\"").collect();
                parse.push(format!("{}", chs[1]));
            }
           _ => {}
        }
    }

    for token in &parse{
        if token == "Error" {
            continue;
        }
        println!("{}",token)
    }
    if had_error {
        process::exit(65);
    }
}
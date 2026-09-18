pub mod json_type;
pub mod json_lexer;
pub mod json_parser;

use std::env;
use std::fs;
fn main () {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];
    let json_string = fs::read_to_string(file_path).expect("Was not able to read the file");
    dbg!(json_parser::parse_json(json_string));
}
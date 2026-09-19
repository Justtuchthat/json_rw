pub mod json_type;
pub mod json_lexer;
pub mod json_parser;
pub mod json_writer;

use core::panic;
use std::env;
use std::fs;

use crate::json_parser::ParseOutput;
fn main () -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {panic!("Not enough arguments passed into ")}
    let in_file_path = &args[1];
    let out_file_path = &args[2];
    let json_string = fs::read_to_string(in_file_path).expect("Was not able to read the file");
    let json = json_parser::parse_json(json_string);
    let json = match json {
        ParseOutput::JSONtype(json) => json,
        ParseOutput::LexError => panic!("Error in lexer!"),
        ParseOutput::ParseError(val) => panic!("Error in parser! {val}"),
    };
    let json_string = json_writer::write_json(json);
    fs::write(out_file_path, json_string)?;
    Ok(())
}
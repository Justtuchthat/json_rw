use crate::json_lexer::{LexOutput, lex, Lexeme};
use crate::json_type::JSONtype;
use std::collections::HashMap;

pub fn parse_json(json_string: String) -> Result<JSONtype, String> {
    /* JSON grammar:
  x JSON = Value
  x Value = Object
  x Value = List
  x Value = Float
  x Value = Int
  x Value = String
  x Value = Bool
  x Value = Null
  x Object = OpenBracket + ObjectFields + CloseBracket
  x ObjectFields = null/empty
  x ObjectFields = ObjectField + Comma + ObjectFields
  x ObjectFields = ObjectField
  x ObjectField = String + Colon + Value
  x List = OpenBracket + ListItems + CloseBracket
  x ListItems = null/empty
  x ListItems = Value + Comma + ListItems
  x ListItems = Value*/
    let mut i: usize = 0;
    let lexemes = match lex(json_string) {
        LexOutput::Lexemes(lexemes) => lexemes,
        LexOutput::StringNotTerminated => {
            return Err(String::from("String not terminated"));
        }
        LexOutput::MultipleDecimalsInFloat => {
            return Err(String::from("Multiple decimal points in float"));
        }
        LexOutput::UnknownCharacter => {
            return Err(String::from("Unknown character in json file"));
        }
    };
    let out = parse_value(&lexemes, &mut i);
    // return out;
    if i != lexemes.len() {
        Err(String::from("Unused lexemes left over"))
    } else {
        out
    }
}

fn parse_value(lexemes: &Vec<Lexeme>, i: &mut usize) -> Result<JSONtype, String> {
    match &lexemes[*i] {
        Lexeme::Null => {
            *i += 1;
            Ok(JSONtype::Null)
        },
        Lexeme::Bool(val) => {
            *i += 1;
            Ok(JSONtype::Bool(*val))
        },
        Lexeme::Int(val) => {
            *i += 1;
            Ok(JSONtype::Int(*val))
        },
        Lexeme::Float(val) => {
            *i += 1;
            Ok(JSONtype::Float(*val))
        },
        Lexeme::String(val) => {
            *i += 1;
            Ok(JSONtype::String(String::from(val)))
        },
        Lexeme::OpenBracket => {
            parse_list(lexemes, i)
        },
        Lexeme::OpenBrace => {
            parse_object(lexemes, i)
        },
        Lexeme::CloseBracket => {
            Err(String::from("Unexpected character ']'"))
        },
        Lexeme::CloseBrace => {
            Err(String::from("Unexpected character '}'"))
        },
        Lexeme::Comma => {
            Err(String::from("Unexpected character ','"))
        },
        Lexeme::Colon => {
            Err(String::from("Unexpected character ':'"))
        },
    }
}

fn parse_list(lexemes: &Vec<Lexeme>, i: &mut usize) -> Result<JSONtype, String> {
    match lexemes[*i] {
        Lexeme::OpenBracket => { *i += 1; }
        _ => {
            return Err(String::from("Expected open bracket"));
        }
    };
    // case of empty list
    if let Lexeme::CloseBracket = lexemes[*i] {
        *i += 1;
        return Ok(JSONtype::List(Vec::new()));
    };
    let list_items = parse_list_items(lexemes, i);
    match list_items {
        Ok(_) => {
            match lexemes[*i] {
                Lexeme::CloseBracket => {
                    *i += 1;
                    list_items
                }
                _ => Err(String::from("Expected close bracket"))
            }
        },
        _ => list_items
    }
}

// only returns error message, or Ok(JSONtype::List)
fn parse_list_items(lexemes: &Vec<Lexeme>, i: &mut usize) -> Result<JSONtype, String> {
    let value = parse_value(lexemes, i);
    match value {
        Ok(json) => {
            // make first value into list
            let json_vec = vec![json];
            match lexemes[*i] {
                // comma means expect more list items
                Lexeme::Comma => {
                    *i += 1; // advance to next lexeme and parse list items again
                    let next_list_items = parse_list_items(lexemes, i);
                    match next_list_items {
                        // if output of parse_list_items is not a jsontype return the error
                        Ok(next_json) => {
                            match next_json {
                                JSONtype::List(next_json_vec) => {
                                    // return concatinated list with the first item, and list of all following items.
                                    Ok(JSONtype::List([json_vec, next_json_vec].concat()))
                                },
                                // if next item is not list, return an error, we should never hit this branch
                                _ => Err(String::from("Should not be possible: parse_list_items returned not a list"))
                            }
                        }
                        _ => next_list_items
                    }
                },
                // no comma, so last list item
                _ => Ok(JSONtype::List(json_vec))
            }
        }
        _ => value
    }
}

fn parse_object(lexemes: &Vec<Lexeme>, i: &mut usize) -> Result<JSONtype, String> {
    match lexemes[*i] {
        Lexeme::OpenBrace => { *i += 1; }
        _ => {
            return Err(String::from("Expected open brace"));
        }
    };
    // case of empty object
    if let Lexeme::CloseBrace = lexemes[*i] {
        *i += 1;
        return Ok(JSONtype::Object(HashMap::new()));
    };
    let object_items = parse_object_items(lexemes, i);
    match object_items {
        Ok(_) => {
            match lexemes[*i] {
                Lexeme::CloseBrace => {
                    *i += 1;
                    object_items
                }
                _ => {
                    Err(String::from("Expected close brace"))
                }
            }
        },
        _ => object_items
    }
}

// only returns error message, or Ok(JSONtype::Object)
fn parse_object_items(lexemes: &Vec<Lexeme>, i: &mut usize) -> Result<JSONtype, String> {
    let json_object = parse_object_item(lexemes, i);
    match &json_object {
        Ok(json_map) => {
            match lexemes[*i] {
                Lexeme::Comma => {
                    *i += 1;
                    if let JSONtype::Object(map) = json_map {
                        let next_json_object = parse_object_items(lexemes, i);
                        match next_json_object {
                            Ok(next_json_map) => {
                                match next_json_map {
                                    JSONtype::Object(mut next_map) => {
                                        for (key, value) in map.iter() {
                                            next_map.insert(String::from(key), value.clone());
                                        }
                                        Ok(JSONtype::Object(next_map))
                                    }
                                    _ => Err(String::from("Should not be reached, parse_object_items returned something other than a hashmap"))
                                }
                            }
                            _ => next_json_object
                        }
                    } else {
                        Err(String::from("Should not be reached, parse_object_item did not return a map"))
                    }
                }
                _ => json_object
            }
        }
        _ => json_object
    }
}

fn parse_object_item(lexemes: &Vec<Lexeme>, i: &mut usize) -> Result<JSONtype, String> {
    match &lexemes[*i] {
        Lexeme::String(key) => {
            *i += 1;
            match lexemes[*i] {
                Lexeme::Colon => {
                    *i += 1;
                    let next_val = parse_value(lexemes, i);
                    match next_val {
                        Ok(value) => {
                            let mut map: HashMap<String, JSONtype> = HashMap::new();
                            map.insert(String::from(key), value);
                            Ok(JSONtype::Object(map))
                        }
                        _ => next_val
                    }
                }
                _ => Err(String::from("Expected colon"))
            }
        }
        _ => Err(String::from("Expected string"))
    }
}
use string_builder::Builder;

#[derive(Debug)]
pub enum Lexeme{
    String(String), Float(f64), Int(i64), Bool(bool),
    OpenBrace, CloseBrace,
    OpenBracket, CloseBracket,
    Colon, Comma, Null
}

pub enum LexOutput {
    Lexemes(Vec<Lexeme>),
    StringNotTerminated,
    MultipleDecimalsInFloat,
    UnknownCharacter,
}

pub fn lex(string: String) -> LexOutput {
    let mut lexemes: Vec<Lexeme> = Vec::new();
    let chars: Vec<char> = string.chars().collect();
    let len = chars.len();
    let mut i: usize = 0;
    while i < len {
        lexemes.push(match chars[i] {
            '\t' => {i += 1; continue;},
            '\n' => {i += 1; continue;},
            ' ' => {i += 1; continue;},
            ':' => {i += 1; Lexeme::Colon},
            ',' => {i += 1; Lexeme::Comma},
            '[' => {i += 1; Lexeme::OpenBracket},
            ']' => {i += 1; Lexeme::CloseBracket},
            '{' => {i += 1; Lexeme::OpenBrace},
            '}' => {i += 1; Lexeme::CloseBrace},
            '"' => {
                // skip to next char of string
                i += 1;
                let mut string_complete = false;
                // setup the variable for the generated string
                let mut final_string: String = String::new();
                // create a string builder
                let mut s_builder = Builder::default();
                // start a new loop
                while i < len {
                    // if char is an escaping backslash, add it and the next char to the list, then continue
                    if chars[i] == '\\' {
                        s_builder.append(chars[i]);
                        i += 1;
                        if i == len {return LexOutput::StringNotTerminated};
                        s_builder.append(chars[i]);
                        i += 1;
                        continue;
                    }
                    // If char is a quote, it cannot be before a backslash, therefore finalize string builder 
                    // and exit while ensuring string complete is set to true
                    if chars[i] == '"' {
                        i += 1;
                        final_string = s_builder.string().unwrap();
                        string_complete = true;
                        break;
                    }
                    // Add char to string builder and next char
                    s_builder.append(chars[i]);
                    i += 1;
                }
                // If string complete is not set, end of file String is reached, and thus return error
                if !string_complete {
                    return LexOutput::StringNotTerminated;
                }
                Lexeme::String(final_string)
            }
            '1' ..= '9' | '-' => {
                let mut s_builder = Builder::default();
                s_builder.append(chars[i]);
                i += 1;
                let mut num_float = false;
                while i < len {
                    if chars[i] == '.' {
                        if num_float {return LexOutput::MultipleDecimalsInFloat};
                        num_float = true;
                        s_builder.append(chars[i]);
                        i += 1;
                    }
                    if "0123456789".contains(chars[i]) {
                        s_builder.append(chars[i]);
                        i += 1;
                    } else {
                        break;
                    }
                }
                if num_float {
                    Lexeme::Float(s_builder.string().unwrap().parse().unwrap())
                } else {
                    Lexeme::Int(s_builder.string().unwrap().parse().unwrap())
                }
            }
            't' => {
                if (i + 3 < len) & (chars[i+1] == 'r') & (chars[i+2] == 'u') & (chars[i+3] == 'e') {
                    i += 4;
                    Lexeme::Bool(true)
                } else {
                    return LexOutput::UnknownCharacter;
                }
            }
            'f' => {
                if (i + 4 < len) & (chars[i+1] == 'a') & (chars[i+2] == 'l') & (chars[i+3] == 's') & (chars[i+4] == 'e') {
                    i += 5;
                    Lexeme::Bool(false)
                } else {
                    return LexOutput::UnknownCharacter;
                }
            }
            'n' => {
                if (i + 3 < len) & (chars[i+1] == 'u') & (chars[i+2] == 'l') & (chars[i+3] == 'l') {
                    i += 4;
                    Lexeme::Null
                } else {
                    return LexOutput::UnknownCharacter;
                }
            }
            _ => {
                return LexOutput::UnknownCharacter;
            }
        });
    }
    LexOutput::Lexemes(lexemes)
}
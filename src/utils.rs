use std::{mem, ops};

#[derive(Clone, Copy)]
pub(crate) struct FnPtr<T>(pub fn(&mut T, Vec<String>));

impl<T> std::fmt::Debug for FnPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&(self.0 as usize as *const ()), f)
    }
}

impl<T> ops::Deref for FnPtr<T> {
    type Target = fn(&mut T, Vec<String>);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn parse(input: String) -> Vec<String> {
    #[derive(Debug, Clone, Default)]
    struct ParserState {
        in_literal: bool,
        last_char: Option<char>,
        last_quote: Option<char>,
        parsed_args: Vec<String>,
        current_arg: String,
    }
    let mut result = input
        .chars()
        .fold(ParserState::default(), |mut state, chr| {
            match state.in_literal {
                true => {
                    if state.last_char != Some('\\') && state.last_quote == Some(chr) {
                        state.in_literal = false;
                        state.last_quote = None;
                        if !state.current_arg.is_empty() {
                            state.parsed_args.push(mem::take(&mut state.current_arg));
                        }
                    } else if state.last_char == Some('\\') {
                        state.current_arg.push(match chr {
                            't' => '\t',
                            'n' => '\n',
                            '"' => '"',
                            '\'' => '\'',
                            '\\' => '\\',
                            _ => panic!("unknown escape character {:?} in input", chr),
                        })
                    } else if chr != '\\' {
                        state.current_arg.push(chr)
                    }
                }
                false => {
                    if chr == ' ' {
                        if !state.current_arg.is_empty() {
                            state.parsed_args.push(mem::take(&mut state.current_arg));
                        }
                    } else if state.last_char == Some('\\') {
                        state.current_arg.push(if chr == 't' {
                            '\t'
                        } else if chr == 'n' {
                            '\n'
                        } else if chr == '"' {
                            '"'
                        } else if chr == '\'' {
                            '\''
                        } else if chr == '\\' {
                            '\\'
                        } else {
                            panic!("unknown escape character {:?} in input", chr);
                        })
                    } else if (chr == '"' || chr == '\'') && state.last_char != Some('\\') {
                        state.in_literal = true;
                        state.last_quote = Some(chr);
                        if !state.current_arg.is_empty() {
                            state.parsed_args.push(mem::take(&mut state.current_arg));
                        }
                    } else if chr != '\\' {
                        state.current_arg.push(chr)
                    } else {
                        state.current_arg.push(chr);
                    }
                }
            }
            state.last_char = Some(chr);
            state
        });
    result.parsed_args.push(mem::take(&mut result.current_arg));
    if result.in_literal {
        panic!("non matching quote characters found!");
    }
    result.parsed_args
}

#[cfg(test)]
mod tests {
    use super::parse;
    use std::panic::catch_unwind;
    #[test]
    fn test_parse() {
        assert_eq!(
            parse(String::from(r#""hello world" world"#)),
            vec!["hello world", "world"]
        );
        assert_eq!(
            parse(String::from(r#"hello world"#)),
            vec!["hello", "world"]
        );
        assert_eq!(
            parse(String::from(r#""hello\"" world"#)),
            vec!["hello\"", "world"]
        );
        assert!(catch_unwind(|| dbg!(parse(String::from(r#"\hello"#)))).is_err());
    }
}

use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Stack<Value>,
    dictionary: HashMap<String, Vec<String>>,
}

pub struct Stack<T> {
    array: Vec<T>,
}

pub enum Token<'a> {
    Word(&'a str),
    Number(&'a str),
    WhiteSpace(&'a str),
}

pub struct Tokenizer<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    fn remaining(&self) -> &'a str {
        &self.source[self.cursor..]
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining();
        if remaining.is_empty() {
            return None;
        }

        let mut chars = remaining.chars();
        let curr_char = chars.next();

        if let Some(c) = curr_char {
            // NEW: peek ahead to detect a leading '-' directly followed by a digit
            let is_negative_number_start =
                c == '-' &&
                remaining[c.len_utf8()..]
                    .chars()
                    .next()
                    .map_or(false, |n| n.is_ascii_digit());

            if c.is_ascii_digit() || is_negative_number_start {
                let mut len = c.len_utf8();
                let start_pos = self.cursor;
                for next_c in chars {
                    if next_c.is_ascii_digit() {
                        len += next_c.len_utf8();
                    } else {
                        break;
                    }
                }
                self.cursor += len;
                let text = &self.source[start_pos..self.cursor];
                return Some(Token::Number(text));
            } else if c.is_whitespace() {
                let mut len = c.len_utf8();
                let start_pos = self.cursor;
                for next_c in chars {
                    if next_c.is_whitespace() {
                        len += next_c.len_utf8();
                    } else {
                        break;
                    }
                }
                self.cursor += len;
                let text = &self.source[start_pos..self.cursor];
                return Some(Token::WhiteSpace(text));
            } else {
                let mut len = c.len_utf8();
                let start_pos = self.cursor;
                for next_c in chars {
                    if next_c.is_whitespace() {
                        break;
                    } else if next_c.is_ascii_digit() {
                        break;
                    } else {
                        len += next_c.len_utf8();
                    }
                }
                self.cursor += len;
                let text = &self.source[start_pos..self.cursor];
                return Some(Token::Word(text));
            }
        }
        None
    }
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { array: Vec::new() }
    }

    fn push(&mut self, _element: T) {
        self.array.push(_element);
    }

    fn pop(&mut self) -> Option<T> {
        return self.array.pop();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Stack::new(),
            dictionary: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.array.as_ref()
    }

    fn expand_word_tokens(&self, raw: &[String]) -> Vec<String> {
        let mut expanded = Vec::new();
        for tok in raw {
            if tok.parse::<Value>().is_ok() {
                expanded.push(tok.clone());
            } else {
                let upper = tok.to_uppercase();
                if let Some(existing) = self.dictionary.get(&upper) {
                    // splice in the CURRENT (already-expanded) definition
                    expanded.extend(existing.clone());
                } else {
                    // built-in or not-yet-defined word: keep as a literal name,
                    // resolved at call time
                    expanded.push(upper);
                }
            }
        }
        expanded
    }

    fn run_word(&mut self, s: &str) -> Result {
        let upper = s.to_uppercase();

        if let Some(def) = self.dictionary.get(&upper).cloned() {
            for tok in &def {
                self.run_token_str(tok)?;
            }
            return Ok(());
        }

        match upper.as_str() {
            "+" => {
                if let Some(b) = self.stack.pop() {
                    if let Some(a) = self.stack.pop() {
                        let result = a + b;
                        self.stack.push(result);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
            "-" => {
                if let Some(b) = self.stack.pop() {
                    if let Some(a) = self.stack.pop() {
                        let result = a - b;
                        self.stack.push(result);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
            "*" => {
                if let Some(b) = self.stack.pop() {
                    if let Some(a) = self.stack.pop() {
                        let result = a * b;
                        self.stack.push(result);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
            "/" => {
                if let Some(b) = self.stack.pop() {
                    if b == 0 {
                        return Err(Error::DivisionByZero);
                    } else {
                        if let Some(a) = self.stack.pop() {
                            let result = a / b;
                            self.stack.push(result);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
            "DUP" => {
                if let Some(a) = self.stack.pop() {
                    self.stack.push(a);
                    self.stack.push(a);
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
            "DROP" => {
                if let Some(_a) = self.stack.pop() {
                    // discarded on purpose
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
            "SWAP" => {
                if let Some(b) = self.stack.pop() {
                    if let Some(a) = self.stack.pop() {
                        self.stack.push(b);
                        self.stack.push(a);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
            "OVER" => {
                if let Some(b) = self.stack.pop() {
                    if let Some(a) = self.stack.pop() {
                        self.stack.push(a);
                        self.stack.push(b);
                        self.stack.push(a);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                } else {
                    return Err(Error::StackUnderflow);
                }
            }
            _ => {
                return Err(Error::UnknownWord);
            }
        }
        Ok(())
    }

    fn run_token_str(&mut self, s: &str) -> Result {
        if let Ok(n) = s.parse::<Value>() {
            self.stack.push(n);
            Ok(())
        } else {
            self.run_word(s)
        }
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = Tokenizer::new(input).filter(|t| !matches!(t, Token::WhiteSpace(_)));

        while let Some(t) = tokens.next() {
            match t {
                Token::Number(s) => {
                    self.stack.push(s.parse().unwrap());
                }
                Token::WhiteSpace(_) => {
                    continue;
                }
                Token::Word(s) => {
                    if s == ":" {
                        let name = match tokens.next() {
                            Some(Token::Word(w)) => w.to_uppercase(),
                            _ => {
                                return Err(Error::InvalidWord);
                            }
                        };
                        if name.parse::<Value>().is_ok() {
                            return Err(Error::InvalidWord);
                        }

                        let mut def: Vec<String> = Vec::new();
                        loop {
                            match tokens.next() {
                                Some(Token::Word(w)) if w == ";" => {
                                    break;
                                }
                                Some(Token::Word(w)) => def.push(w.to_string()),
                                Some(Token::Number(n)) => def.push(n.to_string()),
                                Some(Token::WhiteSpace(_)) => {
                                    continue;
                                }
                                None => {
                                    return Err(Error::InvalidWord);
                                }
                            }
                        }

                        let expanded = self.expand_word_tokens(&def);
                        self.dictionary.insert(name, expanded);
                    } else {
                        self.run_word(s)?;
                    }
                }
            }
        }
        Ok(())
    }
}

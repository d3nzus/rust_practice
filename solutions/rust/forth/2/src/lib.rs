use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

enum Instr {
    Num(Value),
    Word(Rc<Vec<Instr>>),

    Name(String),
}

pub struct Forth {
    stack: Stack<Value>,
    dictionary: HashMap<String, Rc<Vec<Instr>>>,
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

    fn compile_tokens(&self, raw: &[String]) -> Rc<Vec<Instr>> {
        let mut compiled = Vec::new();
        for tok in raw {
            if let Ok(n) = tok.parse::<Value>() {
                compiled.push(Instr::Num(n));
            } else {
                let upper = tok.to_uppercase();
                if let Some(existing) = self.dictionary.get(&upper) {
                    compiled.push(Instr::Word(Rc::clone(existing)));
                } else {
                    compiled.push(Instr::Name(upper));
                }
            }
        }
        Rc::new(compiled)
    }

    fn run_instrs(&mut self, instrs: &Rc<Vec<Instr>>) -> Result {
        let instrs = Rc::clone(instrs);
        for instr in instrs.iter() {
            match instr {
                Instr::Num(n) => self.stack.push(*n),
                Instr::Word(body) => self.run_instrs(body)?,
                Instr::Name(name) => self.run_name(name)?,
            }
        }
        Ok(())
    }

    fn run_name(&mut self, upper: &str) -> Result {
        if let Some(body) = self.dictionary.get(upper).cloned() {
            return self.run_instrs(&body);
        }
        self.run_builtin(upper)
    }

    fn run_builtin(&mut self, upper: &str) -> Result {
        match upper {
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

                        let compiled = self.compile_tokens(&def);
                        self.dictionary.insert(name, compiled);
                    } else {
                        self.run_name(&s.to_uppercase())?;
                    }
                }
            }
        }
        Ok(())
    }
}

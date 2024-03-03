#![allow(dead_code, unused)]
pub enum Token<'a> {
    Function,
    Identifier(&'a str),
    EndOfFile,
}

pub struct DataStream<'a> {
    data: &'a Vec<u8>,
    cursor: usize,
}

impl<'a> DataStream<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        Self { data, cursor: 0 }
    }

    pub fn step_once(&mut self) {
        self.cursor += 1;
    }

    pub fn is_eof(&self) -> bool {
        self.cursor >= self.data.len()
    }

    pub fn selected_value(&self) -> u8 {
        self.data[self.cursor]
    }

    pub fn slice(&self, start: usize) -> &[u8] {
        &self.data[start..self.cursor]
    }
}

pub struct Lexer<'a> {
    stream: DataStream<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        Self { stream: DataStream::new(data) }
    }

    pub fn next_token(&mut self) -> Token {
        self.stream.step_once();
        if self.stream.is_eof() {
            Token::EndOfFile
        } else {
            let char = self.stream.selected_value();
            match char {
                b'a'..=b'z' | b'A'..=b'Z' => {
                    let keyword = std::str::from_utf8(
                        self.dummy_keyword_func()
                    ).unwrap();
                    self.keyword_into_token(keyword)
                },
                _ => unreachable!()
            }
        }
    }

    fn keyword_into_token(&self, keyword: &'a str) -> Token {
        match keyword {
            "function" => Token::Function,
            _ => Token::Identifier(keyword)
        }
    }

    fn dummy_keyword_func(&mut self) -> &[u8] {
        /* don't want to write out the whole function here */
        /* mut because we have to step stream matching a pattern */
        let start = self.stream.cursor;
        self.stream.step_once();
        self.stream.slice(start)
    }
}

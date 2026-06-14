use crate::token::token::Token;

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub next_position: usize,
    pub char: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            next_position: 0,
            char: 0,
        };
        lexer.read_char();
        lexer
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.char {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'*' => Token::MULTIPLY,
            b'/' => Token::DIVIDE,
            b'\\' => Token::BACKSLASH,
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::LESSERTHENEQ
                } else {
                    Token::LESSTHAN
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::GREATERTHANEQ
                } else {
                    Token::GREATERTHAN
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NOTEQUAL
                } else {
                    Token::BANG
                }
            }
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::OPENBRACKET,
            b')' => Token::CLOSEBRACKET,
            b'{' => Token::OPENCURLBRACKET,
            b'}' => Token::CLOSEDCURLBRACKET,
            b'[' => Token::OPENCORNERBRACKET,
            b']' => Token::CLOSEDCORNERBRACKET,
            b':' => Token::COLON,
            b'"' => {
                let string = self.read_string();
                Token::STRING(string)
            }
            0 => Token::EOF("EOF".to_string()),
            _ => {
                return if is_letter(self.char) {
                    let identifier = self.read_identifier();
                    Token::lookup_identifier(&identifier)
                } else if self.char.is_ascii_digit() {
                    let number = self.read_number();
                    Token::INT(number)
                } else {
                    let token = Token::ILLEGAL(String::from_utf8_lossy(&[self.char]).to_string());
                    self.read_char();
                    token
                };
            }
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        let start_position = self.position;
        let end_position: usize;

        loop {
            if self.char == u8::try_from('"').unwrap() || self.char == 0 {
                end_position = self.position;
                break;
            }
            self.read_char();
        }

        self.input[start_position..end_position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.char) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.char.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_char(&mut self) {
        if self.next_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input.as_bytes()[self.next_position];
        }
        self.position = self.next_position;
        self.next_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.next_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.next_position]
        }
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

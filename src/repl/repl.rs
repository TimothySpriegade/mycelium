use crate::lexer::lexer::Lexer;
use crate::token::token::Token;
use std::io::{self, BufRead, Write};
const PROMPT: &str = ">> ";

pub fn start(in_stream: &mut impl BufRead, out_stream: &mut impl Write) -> io::Result<()> {
    let mut line = String::new();

    loop {
        write!(out_stream, "{}", PROMPT)?;
        out_stream.flush()?;

        line.clear();
        let bytes_read = in_stream.read_line(&mut line)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let mut lexer = Lexer::new(line.clone());
        loop {
            let tok = lexer.next_token();
            match tok {
                Token::EOF(_) => break,
                _ => writeln!(out_stream, "{:?}", tok)?,
            }
        }
    }
}

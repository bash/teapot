use std::io::Read;
use std::string::FromUtf8Error;

use super::super::lines::{Lines, LinesError};
use super::super::headers::{Headers, RawHeader};
use super::{is_token, is_whitespace, is_control};

const ASCII_COLON: u8 = 58;

#[derive(Debug)]
enum ParseState {
    BeforeName,
    Name,
    BeforeLine,
    BeforeValue,
    Value,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedCharacterError(u8),
    LineParseError(LinesError),
    FromUtf8Error(FromUtf8Error),
}

type ParseResult = Result<ParseState, ParseError>;

impl From<LinesError> for ParseError {
    fn from(err: LinesError) -> Self {
        ParseError::LineParseError(err)
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(err: FromUtf8Error) -> Self {
        ParseError::FromUtf8Error(err)
    }
}

#[derive(Debug)]
pub struct Parser {
    state: ParseState,
    headers: Headers,
    name: Vec<u8>,
    value: Vec<u8>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: ParseState::BeforeName,
            headers: Headers::new(),
            name: vec![],
            value: vec![],
        }
    }

    ///
    /// Parses message headers according to [`RFC2616 Section 4.2`]
    ///
    /// [`RFC2616 Section 4.2`]: https://tools.ietf.org/html/rfc2616#section-4.2
    // TODO: use dedicated iterator that stops on \r\n\r\n instead of `Lines`
    pub fn parse<T: Read>(&mut self, input: Lines<T>) -> Result<(), ParseError> {
        for line in input {
            for byte in line? {
                match self.process(byte) {
                    Ok(state) => self.state = state,
                    Err(err) => return Err(err),
                }
            }

            self.state = ParseState::BeforeLine;
        }

        self.commit()?;

        Ok(())
    }

    fn process(&mut self, byte: u8) -> ParseResult {
        match self.state {
            ParseState::BeforeName => self.handle_before_name(byte),
            ParseState::Name => self.handle_name(byte),
            ParseState::BeforeValue => self.handle_before_value(byte),
            ParseState::Value => self.handle_value(byte),
            ParseState::BeforeLine => self.handle_before_line(byte),
        }
    }

    fn commit(&mut self) -> Result<(), ParseError> {
        if self.name.len() == 0 {
            return Ok(());
        }

        // TODO: do i really have to clone these?
        let name = self.name.clone();
        let value = self.value.clone();

        self.headers
            .append_raw(RawHeader::new(String::from_utf8(name)?, String::from_utf8(value)?));

        self.name = vec![];
        self.value = vec![];

        Ok(())
    }

    fn handle_before_name(&mut self, byte: u8) -> ParseResult {
        self.commit()?;

        self.consume_name(byte)
    }

    fn handle_name(&mut self, byte: u8) -> ParseResult {
        if byte == ASCII_COLON {
            return Ok(ParseState::BeforeValue);
        }

        self.consume_name(byte)
    }

    fn consume_name(&mut self, byte: u8) -> ParseResult {
        if !is_token(byte) {
            return Err(ParseError::UnexpectedCharacterError(byte));
        }

        self.name.push(byte);

        Ok(ParseState::Name)
    }

    fn handle_before_value(&mut self, byte: u8) -> ParseResult {
        match is_whitespace(byte) {
            true => Ok(ParseState::BeforeValue),
            false => self.consume_value(byte),
        }
    }

    fn handle_value(&mut self, byte: u8) -> ParseResult {
        self.consume_value(byte)
    }

    fn consume_value(&mut self, byte: u8) -> ParseResult {
        if is_control(byte) {
            return Err(ParseError::UnexpectedCharacterError(byte));
        }

        self.value.push(byte);

        Ok(ParseState::Value)
    }

    fn handle_before_line(&mut self, byte: u8) -> ParseResult {
        match is_whitespace(byte) {
            true => Ok(ParseState::BeforeValue),
            false => self.handle_before_name(byte),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::lines::ReadLines;

    #[test]
    fn test_parse() {
        let lines =
            "Host: example.com\r\nUser-Agent: curl/7.51.0\r\nAccept: */*".as_bytes().lines();
        let mut parser = Parser::new();
        let result = parser.parse(lines);

        println!("{:?}", parser);
        println!("{:?}", result);
    }
}

use std::io::Read;
use std::string::FromUtf8Error;
use std::mem::replace;

use super::super::lines::{Lines, LinesError};
use super::super::headers::{Headers, RawHeader};
use super::{is_token, is_whitespace, is_control};

const ASCII_COLON: u8 = 58;
const ASCII_COMMA: u8 = 44;

#[derive(Debug, PartialEq, Eq)]
enum State {
    BeforeName,
    Name,
    NewLine,
    BeforeValue,
    Value,
}

enum Op {
    None,
    AppendName(u8),
    AppendValue(u8)
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedCharacterError(u8),
    LineParseError(LinesError),
    FromUtf8Error(FromUtf8Error),
}

type ParseResult = Result<(Op, State), ParseError>;

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
    state: State,
    headers: Headers,
    name: Vec<u8>,
    value: Vec<u8>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: State::BeforeName,
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
    pub fn parse<T: Read>(&mut self, input: Lines<T>) -> Result<Headers, ParseError> {
        for line in input {
            for byte in line? {
                let (op, state) = self.process(byte)?;

                if state == State::Name && self.state == State::NewLine {
                    self.commit()?;
                }

                self.state = state;
                self.update(op);
            }

            self.state = State::NewLine;
        }

        self.commit()?;

        Ok(replace(&mut self.headers, Headers::new()))
    }

    fn process(&self, byte: u8) -> ParseResult {
        match self.state {
            State::BeforeName => self.before_name(byte),
            State::Name => self.name(byte),
            State::BeforeValue => self.before_value(byte),
            State::Value => self.value(byte),
            State::NewLine => self.new_line(byte),
        }
    }

    fn update(&mut self, op: Op) {
        match op {
            Op::AppendName(byte) => self.name.push(byte),
            Op::AppendValue(byte) => self.value.push(byte),
            Op::None => {},
        }
    }

    fn commit(&mut self) -> Result<(), ParseError> {
        if self.name.len() == 0 {
            return Ok(());
        }

        let mut name = replace(&mut self.name, vec![]);
        let mut value = replace(&mut self.value, vec![]);

        self.headers.append_raw(RawHeader::new(String::from_utf8(name)?, String::from_utf8(value)?));

        Ok(())
    }

    fn before_name(&self, byte: u8) -> ParseResult {
        self.consume_name(byte)
    }

    fn name(&self, byte: u8) -> ParseResult {
        if byte == ASCII_COLON {
            return Ok((Op::None, State::BeforeValue));
        }

        self.consume_name(byte)
    }

    fn consume_name(&self, byte: u8) -> ParseResult {
        if !is_token(byte) {
            return Err(ParseError::UnexpectedCharacterError(byte));
        }

        Ok((Op::AppendName(byte), State::Name))
    }

    fn before_value(&self, byte: u8) -> ParseResult {
        match is_whitespace(byte) {
            true => Ok((Op::None, State::BeforeValue)),
            false => self.consume_value(byte),
        }
    }

    fn value(&self, byte: u8) -> ParseResult {
        self.consume_value(byte)
    }

    fn consume_value(&self, byte: u8) -> ParseResult {
        if is_control(byte) {
            return Err(ParseError::UnexpectedCharacterError(byte));
        }

        Ok((Op::AppendValue(byte), State::Value))
    }

    fn new_line(&self, byte: u8) -> ParseResult {
        match is_whitespace(byte) {
            true => Ok((Op::AppendValue(ASCII_COMMA), State::BeforeValue)),
            false => self.before_name(byte),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::lines::ReadLines;

    #[test]
    fn test_parse() {
        let lines = b"Host: example.com\r\nUser-Agent: curl/7.51.0\r\nAccept: */*".lines();

        let mut parser = Parser::new();
        let headers = parser.parse(lines).unwrap();

        let host = headers.get_raw("host")[0];
        assert_eq!("Host", host.name());
        assert_eq!("example.com", host.value());

        let ua = headers.get_raw("user-agent")[0];
        assert_eq!("User-Agent", ua.name());
        assert_eq!("curl/7.51.0", ua.value());

        let accept = headers.get_raw("accept")[0];
        assert_eq!("Accept", accept.name());
        assert_eq!("*/*", accept.value());
    }
}

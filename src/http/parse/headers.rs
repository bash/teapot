use std;
use std::io::Read;
use std::string::FromUtf8Error;

use super::super::lines::{Lines, LinesError};
use super::super::headers::{Headers, RawHeader};
use super::{is_token, is_whitespace, is_control, ASCII_SPACE, ASCII_COLON};

#[derive(Debug, PartialEq, Eq)]
enum Pos {
    BeforeName,
    Name,
    NewLine,
    BeforeValue,
    Value,
}

#[derive(Debug)]
enum Op {
    None,
    AppendName(u8),
    AppendValue(u8)
}

#[derive(Debug)]
struct State {
    pos: Pos,
    headers: Headers,
    name: Vec<u8>,
    value: Vec<u8>,
}

impl State {
    pub fn new() -> Self {
        State {
            pos: Pos::BeforeName,
            headers: Headers::new(),
            name: vec![],
            value: vec![],
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

        let name = std::mem::replace(&mut self.name, vec![]);
        let value = std::mem::replace(&mut self.value, vec![]);

        self.headers.append_raw(RawHeader::new(String::from_utf8(name)?, String::from_utf8(value)?));

        Ok(())
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedCharacterError(u8),
    LineParseError(LinesError),
    FromUtf8Error(FromUtf8Error),
}

type ParseResult = Result<(Op, Pos), ParseError>;

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

/// Parses message headers according to [`RFC2616 Section 4.2`]
///
/// [`RFC2616 Section 4.2`]: https://tools.ietf.org/html/rfc2616#section-4.2
pub fn parse_headers<R: Read>(input: Lines<R>) -> Result<Headers, ParseError> {
    let mut state = State::new();

    for line in input {
        let line = line?;

        if line.len() == 0 {
            break;
        }

        for byte in line {
            let (op, pos) = process(&state, byte)?;

            if pos == Pos::Name && state.pos == Pos::NewLine {
                state.commit()?;
            }

            state.pos = pos;
            state.update(op);
        }

        state.pos = Pos::NewLine;
    }

    state.commit()?;

    Ok(state.headers)
}

fn process(state: &State, byte: u8) -> ParseResult {
    match state.pos {
        Pos::BeforeName => before_name(byte),
        Pos::Name => name(byte),
        Pos::BeforeValue => before_value(byte),
        Pos::Value => value(byte),
        Pos::NewLine => new_line(byte),
    }
}

fn before_name(byte: u8) -> ParseResult {
    consume_name(byte)
}

fn name(byte: u8) -> ParseResult {
    if byte == ASCII_COLON {
        return Ok((Op::None, Pos::BeforeValue));
    }

    consume_name(byte)
}

fn consume_name(byte: u8) -> ParseResult {
    if !is_token(byte) {
        return Err(ParseError::UnexpectedCharacterError(byte));
    }

    Ok((Op::AppendName(byte), Pos::Name))
}

fn before_value(byte: u8) -> ParseResult {
    match is_whitespace(byte) {
        true => Ok((Op::None, Pos::BeforeValue)),
        false => consume_value(byte),
    }
}

fn value(byte: u8) -> ParseResult {
    consume_value(byte)
}

fn consume_value(byte: u8) -> ParseResult {
    if is_control(byte) {
        return Err(ParseError::UnexpectedCharacterError(byte));
    }

    Ok((Op::AppendValue(byte), Pos::Value))
}

fn new_line(byte: u8) -> ParseResult {
    match is_whitespace(byte) {
        /// According to [`RFC2616 Section 4.2`]:
        /// "... Any LWS that occurs between field-content MAY be replaced
        /// with a single SP before interpreting the field value ..."
        ///
        /// [`RFC2616 Section 4.2`]: https://tools.ietf.org/html/rfc2616#section-4.2
        true => Ok((Op::AppendValue(ASCII_SPACE), Pos::BeforeValue)),
        false => before_name(byte),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::lines::ReadLines;

    #[test]
    fn test_parse() {
        let raw = "Host: example.com\r\nUser-Agent: curl/7.51.0\r\nAccept: */*".as_bytes().lines();
        let headers = parse_headers(raw).unwrap();

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

    #[test]
    fn test_continuation_parse() {
        let raw = "X-Foo: Bar\r\n Qux\r\nX-Baz: Bar".as_bytes().lines();
        let headers = parse_headers(raw).unwrap();

        let foo = headers.get_raw("X-Foo")[0];
        assert_eq!("X-Foo", foo.name());
        assert_eq!("Bar Qux", foo.value());

        let baz = headers.get_raw("X-baz")[0];
        assert_eq!("X-Baz", baz.name());
        assert_eq!("Bar", baz.value());
    }
}

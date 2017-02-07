use std::io::Read;
use std::convert::From;
use std::string::FromUtf8Error;
use super::lines::{ReadLines, LinesError};
use super::headers::{Headers, RawHeader};

#[derive(Debug)]
pub enum ParseError {
    Hello,
    FromUtf8Error(FromUtf8Error),
    ReadError(::std::io::Error),
}

impl From<FromUtf8Error> for ParseError {
    fn from(err: FromUtf8Error) -> Self {
        ParseError::FromUtf8Error(err)
    }
}

impl From<LinesError> for ParseError {
    fn from(err: LinesError) -> Self {
        match err {
            LinesError::ReadError(err) => ParseError::ReadError(err),
        }
    }
}

pub struct Message<'a> {
    start_line: String,
    headers: Headers,
    body: &'a Read,
}

impl<'a> Message<'a> {
    pub fn start_line(&self) -> &String {
        &self.start_line
    }

    pub fn new<S: Into<String>>(start_line: S, headers: Headers, body: &'a Read) -> Self {
        Message {
            start_line: start_line.into(),
            headers: headers,
            body: body,
        }
    }

    pub fn parse(buffer: &mut Read) -> Result<Message, ParseError> {
        let start_line = {
            let mut lines = buffer.lines();

            // TODO: replace .unwrap() with proper error
            let raw = try!(lines.next().unwrap());

            try!(String::from_utf8(raw))
        };

        Ok(Message::new(start_line, Headers::new(), buffer))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let mut bytes = "HTTP/1.1\r\nX-Foo: Bar".as_bytes();
        let message = Message::parse(&mut bytes).unwrap();

        assert_eq!("HTTP/1.1", message.start_line());
    }
}

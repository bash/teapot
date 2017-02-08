use std::io::Read;
use std::convert::From;
use std::string::FromUtf8Error;
use super::lines::{ReadLines, LinesError};
use super::headers::{Headers, RawHeader, ParseError as HeaderParseError};

#[derive(Debug)]
pub enum ParseError {
    Hello,
    FromUtf8Error(FromUtf8Error),
    ReadError(::std::io::Error),
    HeaderParseError(HeaderParseError)
}

impl From<HeaderParseError> for ParseError {
    fn from(err: HeaderParseError) -> Self {
        ParseError::HeaderParseError(err)
    }
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

    pub fn headers(&self) -> &Headers {
        &self.headers
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

        let headers = Headers::parse(buffer.lines())?;

        Ok(Message::new(start_line, headers, buffer))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let mut bytes = "HTTP/1.1\r\nX-Foo: Bar\r\n\r\nFoo".as_bytes();

        {
            let message = Message::parse(&mut bytes).unwrap();

            assert_eq!("HTTP/1.1", message.start_line());

            let foo = message.headers().get_raw("x-foo")[0];
            assert_eq!("X-Foo", foo.name());
            assert_eq!("Bar", foo.value());

            assert_eq!(1, message.headers().len_raw());
        }

        {
            let mut body = String::new();
            bytes.read_to_string(&mut body);
            assert_eq!("Foo", body);
        }
    }
}

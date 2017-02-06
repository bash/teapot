use std::io::{Read};

#[derive(Debug)]
pub enum LinesError {
    ReadError(::std::io::Error)
}

impl From<::std::io::Error> for LinesError {
    fn from(err: ::std::io::Error) -> Self {
        LinesError::ReadError(err)
    }
}

pub struct Lines<R> {
    inner: R
}

impl<R> Lines<R> {
    fn new(inner: R) -> Self {
        Lines { inner: inner }
    }
}

impl<R> Lines<R> where R: Read {
    fn read_byte(&mut self) -> Result<Option<u8>, LinesError> {
        let mut bytes = [0; 1];
        let bytes_read = self.inner.read(&mut bytes)?;

        let byte = match bytes_read {
            0 => None,
            _ => Some(bytes[0])
        };

        Ok(byte)
    }
}

// TODO: clean up
impl<R: Read> Iterator for Lines<R> {
    type Item = Result<Vec<u8>, LinesError>;

    // TODO: isn't there a cleaner way to do this?
    fn next(&mut self) -> Option<Self::Item> {
        let mut prev_byte = 0u8;
        let mut buf: Vec<u8> = vec![];

        loop {
            let mut bytes = [0; 1];
            let bytes_read = try_to_opt!(self.inner.read(&mut bytes));
            let byte = bytes[0];

            if bytes_read == 0 {
                return match buf.len() {
                    0 => None,
                    _ => Some(Ok(buf))
                }
            }

            if prev_byte == 13u8 && byte == 10u8 {
                buf.pop();
                return Some(Ok(buf));
            }

            buf.push(byte);
            prev_byte = byte;
        }
    }
}

pub trait ReadLines {
    fn lines(self) -> Lines<Self> where Self: Sized {
        Lines { inner: self }
    }
}

impl<T> ReadLines for T where T: Read {}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut bytes = "Foo\r\nBar\r\nBaz\r\n\r\nQux".as_bytes();
        let mut iter = Lines::new(bytes);

        assert_eq!("Foo".to_string().into_bytes(), iter.next().unwrap().unwrap());
        assert_eq!("Bar".to_string().into_bytes(), iter.next().unwrap().unwrap());
        assert_eq!("Baz".to_string().into_bytes(), iter.next().unwrap().unwrap());
        assert_eq!("".to_string().into_bytes(), iter.next().unwrap().unwrap());
        assert_eq!("Qux".to_string().into_bytes(), iter.next().unwrap().unwrap());
        assert!(iter.next().is_none());
    }
}
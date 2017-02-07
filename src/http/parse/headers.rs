use super::super::lines::Lines;
use super::super::headers::RawHeader;
use std::io::Read;

/*enum ParserState {
    Error,
    BeforeName,
    Name,
    BeforeLine,
    BeforeValue,
    Value
}

pub struct Parser<'a, T: Read> {
    state: ParserState,
    input: &'a Lines<T>,
    headers: Vec<RawHeader>,
    name: Vec<u8>,
    value: Vec<u8>
}
*/
/*impl<'a, T: Read> Parser<'a, T> {
    pub fn new(input: &'a Lines<Read>) -> Self {
        Parser {
            state: ParserState::BeforeName,
            input: input,
            headers: vec![],
            name: vec![],
            value: vec![]
        }
    }

    pub fn parse(&mut self) {}

    fn next(&mut self) {}
}
*/

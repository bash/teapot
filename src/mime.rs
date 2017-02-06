use std::fmt;
use std::str::FromStr;

// TODO: add more types (maybe automated from https://www.iana.org/assignments/media-types/media-types.xhtml)
pub trait Mime : fmt::Display {
    fn top<'a>() -> &'a str;
    fn sub<'a>() -> &'a str;
}

/*impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}*/

/*impl FromStr for MediaType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {

    }
}
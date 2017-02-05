use std::fmt;

// TODO: add more types (maybe automated from https://www.iana.org/assignments/media-types/media-types.xhtml)
pub enum MediaType {
    Json,
    Html
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            MediaType::Json => "application/json",
            MediaType::Html => "text/html"
        };

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!("application/json", MediaType::Json.to_string());
        assert_eq!("text/html", MediaType::Html.to_string());
    }
}
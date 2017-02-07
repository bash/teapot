use std::collections::BTreeSet;

/// # Examples
///
/// ```
/// use teapot::http::headers::{TypedHeader, RawHeader};
///
/// #[derive(PartialEq, Eq, Debug)]
/// pub struct UserAgentHeader {
///     value: String
/// }
///
/// impl UserAgentHeader {
///     pub fn value(&self) -> &str {
///         &self.value
///     }
/// }
///
/// impl TypedHeader for UserAgentHeader {
///     fn name() -> &'static str {
///         "user-agent"
///     }
///
///     fn parse(raw: &[RawHeader]) -> Option<Self> {
///         if raw.len() == 0 {
///             return None;
///         }
///
///         Some(UserAgentHeader { value: raw[0].value().to_string() })
///     }
/// }
/// ```
pub trait TypedHeader: Eq + Sized {
    /// This is the name of the header in lower case.
    /// It is used by `Headers` to look up the raw header(s).
    fn name() -> &'static str;

    /// Converts a list of raw values to a `TypedHeader`
    /// The list is required for headers like `Set-Cookie` which might appear multiple times in a response.
    /// Other headers might only use the first value of `raw` and ignore the rest
    fn parse(raw: &[RawHeader]) -> Option<Self>;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawHeader<'a> {
    name: &'a str,
    value: &'a str
}

impl<'a> RawHeader<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        RawHeader { name: name, value: value }
    }

    pub fn name(&self) -> &str {
        return &self.name
    }

    pub fn value(&self) -> &str {
        return &self.value
    }
}

pub struct Headers<'a> {
    headers: BTreeSet<RawHeader<'a>>
}

// TODO: allow creation from iterator
impl<'a> Headers<'a> {
    pub fn new() -> Self {
        Headers { headers: BTreeSet::new() }
    }

    pub fn append_raw(&mut self, header: RawHeader<'a>) {
        self.headers.insert(header);
    }

    pub fn get<T: TypedHeader>(&self) -> Option<T> {
        let raw = self.get_raw(T::name());

        T::parse(&raw)
    }

    pub fn get_raw(&self, name: &str) -> Vec<RawHeader> {
        // TODO: not sure if this is good
        self.headers
            .iter()
            .filter(|ref header| *header.name().to_lowercase() == *name)
            .map(|ref header| *header.clone())
            .collect()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Dnt {
    Disabled,
    Enabled,
    Unspecified
}

#[derive(PartialEq, Eq, Debug)]
pub struct DntHeader {
    value: Dnt
}

impl DntHeader {
    pub fn value(&self) -> &Dnt {
        &self.value
    }
}

impl TypedHeader for DntHeader {
    fn name() -> &'static str {
        return "dnt"
    }

    fn parse(raw: &[RawHeader]) -> Option<Self> {
        if raw.len() == 0 {
            return Some(DntHeader { value: Dnt::Unspecified });
        }

        let value = match raw[0].value() {
            "1" => Dnt::Enabled,
            "0" => Dnt::Disabled,
            _ => Dnt::Unspecified
        };

        Some(DntHeader { value: value })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct UserAgentHeader {
    value: String
}

impl UserAgentHeader {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TypedHeader for UserAgentHeader {
    fn name() -> &'static str {
        "user-agent"
    }

    fn parse(raw: &[RawHeader]) -> Option<Self> {
        if raw.len() == 0 {
            return None;
        }

        Some(UserAgentHeader { value: raw[0].value().to_string() })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_raw_header() {
        let header = RawHeader::new("X-Foo", "bar");

        assert_eq!("X-Foo", header.name());
        assert_eq!("bar", header.value());
    }

    #[test]
    fn test_get_header() {
        let mut headers = Headers::new();

        headers.append_raw(RawHeader::new("dnt", "1"));

        let dnt: DntHeader = headers.get().unwrap();

        assert_eq!(Dnt::Enabled, *dnt.value());
    }

    #[test]
    fn test_user_agent() {
        {
            let ua = UserAgentHeader::parse(&[RawHeader::new("user-agent", "foobar/1.1")]);

            assert!(ua.is_some());
            assert_eq!("foobar/1.1", ua.unwrap().value());
        }

        {
            let ua = UserAgentHeader::parse(&[]);

            assert!(ua.is_none());
        }
    }
}
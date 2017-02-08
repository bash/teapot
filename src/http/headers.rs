use std::collections::BTreeSet;
use std::fmt;
use std::io::Read;
use super::parse::headers::parse_headers;
pub use super::parse::headers::ParseError;

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
///     fn canonical_name() -> &'static str {
///        "User-Agent"
///     }
///
///     fn parse(raw: &[&RawHeader]) -> Option<Self> {
///         if raw.len() == 0 {
///             return None;
///         }
///
///         Some(UserAgentHeader { value: raw[0].value().to_string() })
///     }
///
///     fn raw_values(&self) -> Vec<String> {
///       vec![format!("{}", self.value)]
///     }
/// }
/// ```
pub trait TypedHeader: Eq + Sized {
    /// This is the name of the header in its canonical form.
    /// Used by [`to_raw`] and [`get_raw`] as the header's name.
    ///
    /// [`to_raw`]: trait.TypedHeader.html#method.to_raw
    /// [`get`]: struct.Headers.html#method.get
    fn canonical_name() -> &'static str;

    /// Converts a list of raw values to a `TypedHeader`
    /// The list is required for headers like `Set-Cookie` which might appear
    /// multiple times in a response. Other headers might only
    /// use the first value of `raw` and ignore the rest.
    // TODO: consider using Result instead of Option
    // (however we still need to be able to represent the absence of a header)
    fn parse(raw: &[&RawHeader]) -> Option<Self>;

    /// Returns the raw values of this header.
    /// Used by [`to_raw`] as the header value.
    /// When multiple values are returned,
    /// [`to_raw`] will return multiple raw headers for each value.
    ///
    /// [`to_raw`]: trait.TypedHeader.html#method.to_raw
    fn raw_values(&self) -> Vec<String>;

    /// Converts the header back to one or more [`RawHeader`]s
    ///
    /// [`RawHeader`]: struct.RawHeader.html
    fn to_raw(&self) -> Vec<RawHeader> {
        self.raw_values()
            .iter()
            .map(|value| RawHeader::new(Self::canonical_name(), &value))
            .collect()
    }
}

#[derive(Eq, PartialOrd, Ord, Debug)]
pub struct RawHeader {
    name: String,
    value: String,
}

impl RawHeader {
    pub fn new<S: Into<String>>(name: S, value: S) -> Self {
        RawHeader {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PartialEq<RawHeader> for RawHeader {
    fn eq(&self, other: &RawHeader) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
            && self.value == other.value
    }
}

/// # Examples
///
/// ```
/// use teapot::http::headers::{Headers, DntHeader, Dnt};
///
/// let headers = Headers::new();
/// let dnt : DntHeader = headers.get().unwrap();
///
/// assert_eq!(Dnt::Unspecified, dnt.value());
/// ```
#[derive(Debug)]
pub struct Headers {
    headers: BTreeSet<RawHeader>,
}

// TODO: allow creation from iterator
impl Headers {
    pub fn new() -> Self {
        Headers { headers: BTreeSet::new() }
    }

    pub fn parse<R: Read>(input: R) -> Result<Self, ParseError> {
        parse_headers(input)
    }

    pub fn append<H: TypedHeader>(&mut self, header: H) {
        for header in header.to_raw() {
            self.append_raw(header);
        }
    }

    pub fn append_raw(&mut self, header: RawHeader) {
        self.headers.insert(header);
    }

    pub fn get<H: TypedHeader>(&self) -> Option<H> {
        let raw = self.get_raw(H::canonical_name());

        H::parse(raw.as_slice())
    }

    pub fn get_raw(&self, name: &str) -> Vec<&RawHeader> {
        // TODO: ask @SirRade for an opinion on this
        self.headers
            .iter()
            .filter(|ref header| header.name.to_lowercase() == *name.to_lowercase())
            .collect()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Dnt {
    Disabled,
    Enabled,
    Unspecified,
}

impl fmt::Display for Dnt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Dnt::Disabled => "0",
            Dnt::Enabled => "1",
            Dnt::Unspecified => "",
        };

        write!(f, "{}", value)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct DntHeader {
    value: Dnt,
}

impl DntHeader {
    pub fn new(value: Dnt) -> Self {
        DntHeader { value: value }
    }

    pub fn value(&self) -> Dnt {
        self.value
    }
}

impl TypedHeader for DntHeader {
    fn canonical_name() -> &'static str {
        "DNT"
    }

    fn parse(raw: &[&RawHeader]) -> Option<Self> {
        if raw.len() == 0 {
            return Some(DntHeader { value: Dnt::Unspecified });
        }

        let value = match raw[0].value() {
            "1" => Dnt::Enabled,
            "0" => Dnt::Disabled,
            _ => Dnt::Unspecified,
        };

        Some(DntHeader { value: value })
    }

    fn raw_values(&self) -> Vec<String> {
        vec![format!("{}", self.value)]
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct UserAgentHeader {
    value: String,
}

impl UserAgentHeader {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TypedHeader for UserAgentHeader {
    fn canonical_name() -> &'static str {
        "User-Agent"
    }

    fn parse(raw: &[&RawHeader]) -> Option<Self> {
        if raw.len() == 0 {
            return None;
        }

        Some(UserAgentHeader { value: raw[0].value().to_string() })
    }

    fn raw_values(&self) -> Vec<String> {
        vec![self.value.clone()]
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

        assert_eq!(Dnt::Enabled, dnt.value());
    }

    #[test]
    fn test_user_agent() {
        {
            let ua = UserAgentHeader::parse(&[&RawHeader::new("user-agent", "foobar/1.1")]);

            assert!(ua.is_some());
            assert_eq!("foobar/1.1", ua.unwrap().value());
        }

        {
            let ua = UserAgentHeader::parse(&[]);

            assert!(ua.is_none());
        }
    }

    #[test]
    fn test_dnt() {
        assert_eq!("0", format!("{}", Dnt::Disabled));
        assert_eq!("1", format!("{}", Dnt::Enabled));
        assert_eq!("", format!("{}", Dnt::Unspecified));
    }

    #[test]
    fn test_append() {
        let mut headers = Headers::new();

        headers.append(DntHeader::new(Dnt::Enabled));

        let result: DntHeader = headers.get().unwrap();

        assert_eq!(DntHeader::new(Dnt::Enabled), result);
    }
}

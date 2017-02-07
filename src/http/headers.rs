use std::collections::BTreeSet;

pub trait TypedHeader: Eq + Sized {
    fn name() -> &'static str;
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

    fn name(&self) -> &str {
        return &self.name
    }

    fn value(&self) -> &str {
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

    pub fn append(&mut self, header: RawHeader<'a>) {
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
            .filter(|ref header| *header.name() == *name)
            .map(|ref header| *header.clone())
            .collect()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum DntValue {
    Disabled,
    Enabled,
    Unspecified
}

#[derive(PartialEq, Eq, Debug)]
pub struct Dnt {
    value: DntValue
}

impl Dnt {
    pub fn value(&self) -> &DntValue {
        &self.value
    }
}

impl TypedHeader for Dnt {
    fn name() -> &'static str {
        return "DNT"
    }

    fn parse(raw: &[RawHeader]) -> Option<Self> {
        if raw.len() == 0 {
            return Some(Dnt { value: DntValue::Unspecified });
        }

        let value = match raw[0].value() {
            "1" => DntValue::Enabled,
            "0" => DntValue::Disabled,
            _ => DntValue::Unspecified
        };

        Some(Dnt { value: value })
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

        headers.append(RawHeader::new("DNT", "1"));

        let dnt: Dnt = headers.get().unwrap();

        assert_eq!(DntValue::Enabled, *dnt.value());
    }
}
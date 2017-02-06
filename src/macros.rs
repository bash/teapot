#[macro_export]
macro_rules! try_to_opt {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Some(Err(::std::convert::From::from(err))),
    });
}
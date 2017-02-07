#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum StatusCode {
    /// 100 Continue
    /// [RFC7231, Section 6.2.1](http://www.iana.org/go/rfc7231)
    Continue,
    /// 101 Switching Protocols
    /// [RFC7231, Section 6.2.2](http://www.iana.org/go/rfc7231)
    SwitchingProtocols,
    /// 102 Processing
    /// [RFC2518](http://www.iana.org/go/rfc2518)
    Processing,
    /// 200 OK
    /// [RFC7231, Section 6.3.1](http://www.iana.org/go/rfc7231)
    Ok,
    /// 201 Created
    /// [RFC7231, Section 6.3.2](http://www.iana.org/go/rfc7231)
    Created,
    /// 202 Accepted
    /// [RFC7231, Section 6.3.3](http://www.iana.org/go/rfc7231)
    Accepted,
    /// 203 Non-Authoritative Information
    /// [RFC7231, Section 6.3.4](http://www.iana.org/go/rfc7231)
    NonAuthoritativeInformation,
    /// 204 No Content
    /// [RFC7231, Section 6.3.5](http://www.iana.org/go/rfc7231)
    NoContent,
    /// 205 Reset Content
    /// [RFC7231, Section 6.3.6](http://www.iana.org/go/rfc7231)
    ResetContent,
    /// 206 Partial Content
    /// [RFC7233, Section 4.1](http://www.iana.org/go/rfc7233)
    PartialContent,
    /// 207 Multi-Status
    /// [RFC4918](http://www.iana.org/go/rfc4918)
    MultiStatus,
    /// 208 Already Reported
    /// [RFC5842](http://www.iana.org/go/rfc5842)
    AlreadyReported,
    /// 226 IM Used
    /// [RFC3229](http://www.iana.org/go/rfc3229)
    ImUsed,
    /// 300 Multiple Choices
    /// [RFC7231, Section 6.4.1](http://www.iana.org/go/rfc7231)
    MultipleChoices,
    /// 301 Moved Permanently
    /// [RFC7231, Section 6.4.2](http://www.iana.org/go/rfc7231)
    MovedPermanently,
    /// 302 Found
    /// [RFC7231, Section 6.4.3](http://www.iana.org/go/rfc7231)
    Found,
    /// 303 See Other
    /// [RFC7231, Section 6.4.4](http://www.iana.org/go/rfc7231)
    SeeOther,
    /// 304 Not Modified
    /// [RFC7232, Section 4.1](http://www.iana.org/go/rfc7232)
    NotModified,
    /// 305 Use Proxy
    /// [RFC7231, Section 6.4.5](http://www.iana.org/go/rfc7231)
    UseProxy,
    /// 307 Temporary Redirect
    /// [RFC7231, Section 6.4.7](http://www.iana.org/go/rfc7231)
    TemporaryRedirect,
    /// 308 Permanent Redirect
    /// [RFC7538](http://www.iana.org/go/rfc7538)
    PermanentRedirect,
    /// 400 Bad Request
    /// [RFC7231, Section 6.5.1](http://www.iana.org/go/rfc7231)
    BadRequest,
    /// 401 Unauthorized
    /// [RFC7235, Section 3.1](http://www.iana.org/go/rfc7235)
    Unauthorized,
    /// 402 Payment Required
    /// [RFC7231, Section 6.5.2](http://www.iana.org/go/rfc7231)
    PaymentRequired,
    /// 403 Forbidden
    /// [RFC7231, Section 6.5.3](http://www.iana.org/go/rfc7231)
    Forbidden,
    /// 404 Not Found
    /// [RFC7231, Section 6.5.4](http://www.iana.org/go/rfc7231)
    NotFound,
    /// 405 Method Not Allowed
    /// [RFC7231, Section 6.5.5](http://www.iana.org/go/rfc7231)
    MethodNotAllowed,
    /// 406 Not Acceptable
    /// [RFC7231, Section 6.5.6](http://www.iana.org/go/rfc7231)
    NotAcceptable,
    /// 407 Proxy Authentication Required
    /// [RFC7235, Section 3.2](http://www.iana.org/go/rfc7235)
    ProxyAuthenticationRequired,
    /// 408 Request Timeout
    /// [RFC7231, Section 6.5.7](http://www.iana.org/go/rfc7231)
    RequestTimeout,
    /// 409 Conflict
    /// [RFC7231, Section 6.5.8](http://www.iana.org/go/rfc7231)
    Conflict,
    /// 410 Gone
    /// [RFC7231, Section 6.5.9](http://www.iana.org/go/rfc7231)
    Gone,
    /// 411 Length Required
    /// [RFC7231, Section 6.5.10](http://www.iana.org/go/rfc7231)
    LengthRequired,
    /// 412 Precondition Failed
    /// [RFC7232, Section 4.2](http://www.iana.org/go/rfc7232)
    PreconditionFailed,
    /// 413 Payload Too Large
    /// [RFC7231, Section 6.5.11](http://www.iana.org/go/rfc7231)
    PayloadTooLarge,
    /// 414 URI Too Long
    /// [RFC7231, Section 6.5.12](http://www.iana.org/go/rfc7231)
    UriTooLong,
    /// 415 Unsupported Media Type
    /// [RFC7231, Section 6.5.13][RFC7694, Section 3](http://www.iana.org/go/rfc7231)
    UnsupportedMediaType,
    /// 416 Range Not Satisfiable
    /// [RFC7233, Section 4.4](http://www.iana.org/go/rfc7233)
    RangeNotSatisfiable,
    /// 417 Expectation Failed
    /// [RFC7231, Section 6.5.14](http://www.iana.org/go/rfc7231)
    ExpectationFailed,
    /// 421 Misdirected Request
    /// [RFC7540, Section 9.1.2](http://www.iana.org/go/rfc7540)
    MisdirectedRequest,
    /// 422 Unprocessable Entity
    /// [RFC4918](http://www.iana.org/go/rfc4918)
    UnprocessableEntity,
    /// 423 Locked
    /// [RFC4918](http://www.iana.org/go/rfc4918)
    Locked,
    /// 424 Failed Dependency
    /// [RFC4918](http://www.iana.org/go/rfc4918)
    FailedDependency,
    /// 426 Upgrade Required
    /// [RFC7231, Section 6.5.15](http://www.iana.org/go/rfc7231)
    UpgradeRequired,
    /// 428 Precondition Required
    /// [RFC6585](http://www.iana.org/go/rfc6585)
    PreconditionRequired,
    /// 429 Too Many Requests
    /// [RFC6585](http://www.iana.org/go/rfc6585)
    TooManyRequests,
    /// 431 Request Header Fields Too Large
    /// [RFC6585](http://www.iana.org/go/rfc6585)
    RequestHeaderFieldsTooLarge,
    /// 451 Unavailable For Legal Reasons
    /// [RFC7725](http://www.iana.org/go/rfc7725)
    UnavailableForLegalReasons,
    /// 500 Internal Server Error
    /// [RFC7231, Section 6.6.1](http://www.iana.org/go/rfc7231)
    InternalServerError,
    /// 501 Not Implemented
    /// [RFC7231, Section 6.6.2](http://www.iana.org/go/rfc7231)
    NotImplemented,
    /// 502 Bad Gateway
    /// [RFC7231, Section 6.6.3](http://www.iana.org/go/rfc7231)
    BadGateway,
    /// 503 Service Unavailable
    /// [RFC7231, Section 6.6.4](http://www.iana.org/go/rfc7231)
    ServiceUnavailable,
    /// 504 Gateway Timeout
    /// [RFC7231, Section 6.6.5](http://www.iana.org/go/rfc7231)
    GatewayTimeout,
    /// 505 HTTP Version Not Supported
    /// [RFC7231, Section 6.6.6](http://www.iana.org/go/rfc7231)
    HttpVersionNotSupported,
    /// 506 Variant Also Negotiates
    /// [RFC2295](http://www.iana.org/go/rfc2295)
    VariantAlsoNegotiates,
    /// 507 Insufficient Storage
    /// [RFC4918](http://www.iana.org/go/rfc4918)
    InsufficientStorage,
    /// 508 Loop Detected
    /// [RFC5842](http://www.iana.org/go/rfc5842)
    LoopDetected,
    /// 510 Not Extended
    /// [RFC2774](http://www.iana.org/go/rfc2774)
    NotExtended,
    /// 511 Network Authentication Required
    /// [RFC6585](http://www.iana.org/go/rfc6585)
    NetworkAuthenticationRequired,
    /// Catch-all for unregistered status codes
    Unregistered(u16)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum StatusClass {
    /// 1xx: Informational - Request received, continuing process
    Informational,
    /// 2xx: Success - The action was successfully received, understood, and accepted
    Success,
    /// 3xx: Redirection - Further action must be taken in order to complete the request
    Redirection,
    /// 4xx: Client Error - The request contains bad syntax or cannot be fulfilled
    ClientError,
    /// 5xx: Server Error - The server failed to fulfill an apparently valid request
    ServerError,
    /// Catch-All for everything outside the defined classes
    None
}

impl StatusCode {
    pub fn from_u16(value: u16) -> Self {
        match value {
            100 => StatusCode::Continue,
            101 => StatusCode::SwitchingProtocols,
            102 => StatusCode::Processing,
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            203 => StatusCode::NonAuthoritativeInformation,
            204 => StatusCode::NoContent,
            205 => StatusCode::ResetContent,
            206 => StatusCode::PartialContent,
            207 => StatusCode::MultiStatus,
            208 => StatusCode::AlreadyReported,
            226 => StatusCode::ImUsed,
            300 => StatusCode::MultipleChoices,
            301 => StatusCode::MovedPermanently,
            302 => StatusCode::Found,
            303 => StatusCode::SeeOther,
            304 => StatusCode::NotModified,
            305 => StatusCode::UseProxy,
            307 => StatusCode::TemporaryRedirect,
            308 => StatusCode::PermanentRedirect,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            402 => StatusCode::PaymentRequired,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            406 => StatusCode::NotAcceptable,
            407 => StatusCode::ProxyAuthenticationRequired,
            408 => StatusCode::RequestTimeout,
            409 => StatusCode::Conflict,
            410 => StatusCode::Gone,
            411 => StatusCode::LengthRequired,
            412 => StatusCode::PreconditionFailed,
            413 => StatusCode::PayloadTooLarge,
            414 => StatusCode::UriTooLong,
            415 => StatusCode::UnsupportedMediaType,
            416 => StatusCode::RangeNotSatisfiable,
            417 => StatusCode::ExpectationFailed,
            421 => StatusCode::MisdirectedRequest,
            422 => StatusCode::UnprocessableEntity,
            423 => StatusCode::Locked,
            424 => StatusCode::FailedDependency,
            426 => StatusCode::UpgradeRequired,
            428 => StatusCode::PreconditionRequired,
            429 => StatusCode::TooManyRequests,
            431 => StatusCode::RequestHeaderFieldsTooLarge,
            451 => StatusCode::UnavailableForLegalReasons,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            502 => StatusCode::BadGateway,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::HttpVersionNotSupported,
            506 => StatusCode::VariantAlsoNegotiates,
            507 => StatusCode::InsufficientStorage,
            508 => StatusCode::LoopDetected,
            510 => StatusCode::NotExtended,
            511 => StatusCode::NetworkAuthenticationRequired,
            _ => StatusCode::Unregistered(value)
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            StatusCode::Continue => 100,
            StatusCode::SwitchingProtocols => 101,
            StatusCode::Processing => 102,
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::Accepted => 202,
            StatusCode::NonAuthoritativeInformation => 203,
            StatusCode::NoContent => 204,
            StatusCode::ResetContent => 205,
            StatusCode::PartialContent => 206,
            StatusCode::MultiStatus => 207,
            StatusCode::AlreadyReported => 208,
            StatusCode::ImUsed => 226,
            StatusCode::MultipleChoices => 300,
            StatusCode::MovedPermanently => 301,
            StatusCode::Found => 302,
            StatusCode::SeeOther => 303,
            StatusCode::NotModified => 304,
            StatusCode::UseProxy => 305,
            StatusCode::TemporaryRedirect => 307,
            StatusCode::PermanentRedirect => 308,
            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::PaymentRequired => 402,
            StatusCode::Forbidden => 403,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::NotAcceptable => 406,
            StatusCode::ProxyAuthenticationRequired => 407,
            StatusCode::RequestTimeout => 408,
            StatusCode::Conflict => 409,
            StatusCode::Gone => 410,
            StatusCode::LengthRequired => 411,
            StatusCode::PreconditionFailed => 412,
            StatusCode::PayloadTooLarge => 413,
            StatusCode::UriTooLong => 414,
            StatusCode::UnsupportedMediaType => 415,
            StatusCode::RangeNotSatisfiable => 416,
            StatusCode::ExpectationFailed => 417,
            StatusCode::MisdirectedRequest => 421,
            StatusCode::UnprocessableEntity => 422,
            StatusCode::Locked => 423,
            StatusCode::FailedDependency => 424,
            StatusCode::UpgradeRequired => 426,
            StatusCode::PreconditionRequired => 428,
            StatusCode::TooManyRequests => 429,
            StatusCode::RequestHeaderFieldsTooLarge => 431,
            StatusCode::UnavailableForLegalReasons => 451,
            StatusCode::InternalServerError => 500,
            StatusCode::NotImplemented => 501,
            StatusCode::BadGateway => 502,
            StatusCode::ServiceUnavailable => 503,
            StatusCode::GatewayTimeout => 504,
            StatusCode::HttpVersionNotSupported => 505,
            StatusCode::VariantAlsoNegotiates => 506,
            StatusCode::InsufficientStorage => 507,
            StatusCode::LoopDetected => 508,
            StatusCode::NotExtended => 510,
            StatusCode::NetworkAuthenticationRequired => 511,
            StatusCode::Unregistered(value) => value
        }
    }

    pub fn class(&self) -> StatusClass {
        match self.to_u16() {
            100 ... 199 => StatusClass::Informational,
            200 ... 299 => StatusClass::Success,
            300 ... 399 => StatusClass::Redirection,
            400 ... 499 => StatusClass::ClientError,
            500 ... 599 => StatusClass::ServerError,
            _ => StatusClass::None
        }
    }

    pub fn canonical_reason(&self) -> Option<&'static str> {
        match *self {
            StatusCode::Continue => Some("Continue"),
            StatusCode::SwitchingProtocols => Some("Switching Protocols"),
            StatusCode::Processing => Some("Processing"),
            StatusCode::Ok => Some("OK"),
            StatusCode::Created => Some("Created"),
            StatusCode::Accepted => Some("Accepted"),
            StatusCode::NonAuthoritativeInformation => Some("Non-Authoritative Information"),
            StatusCode::NoContent => Some("No Content"),
            StatusCode::ResetContent => Some("Reset Content"),
            StatusCode::PartialContent => Some("Partial Content"),
            StatusCode::MultiStatus => Some("Multi-Status"),
            StatusCode::AlreadyReported => Some("Already Reported"),
            StatusCode::ImUsed => Some("IM Used"),
            StatusCode::MultipleChoices => Some("Multiple Choices"),
            StatusCode::MovedPermanently => Some("Moved Permanently"),
            StatusCode::Found => Some("Found"),
            StatusCode::SeeOther => Some("See Other"),
            StatusCode::NotModified => Some("Not Modified"),
            StatusCode::UseProxy => Some("Use Proxy"),
            StatusCode::TemporaryRedirect => Some("Temporary Redirect"),
            StatusCode::PermanentRedirect => Some("Permanent Redirect"),
            StatusCode::BadRequest => Some("Bad Request"),
            StatusCode::Unauthorized => Some("Unauthorized"),
            StatusCode::PaymentRequired => Some("Payment Required"),
            StatusCode::Forbidden => Some("Forbidden"),
            StatusCode::NotFound => Some("Not Found"),
            StatusCode::MethodNotAllowed => Some("Method Not Allowed"),
            StatusCode::NotAcceptable => Some("Not Acceptable"),
            StatusCode::ProxyAuthenticationRequired => Some("Proxy Authentication Required"),
            StatusCode::RequestTimeout => Some("Request Timeout"),
            StatusCode::Conflict => Some("Conflict"),
            StatusCode::Gone => Some("Gone"),
            StatusCode::LengthRequired => Some("Length Required"),
            StatusCode::PreconditionFailed => Some("Precondition Failed"),
            StatusCode::PayloadTooLarge => Some("Payload Too Large"),
            StatusCode::UriTooLong => Some("URI Too Long"),
            StatusCode::UnsupportedMediaType => Some("Unsupported Media Type"),
            StatusCode::RangeNotSatisfiable => Some("Range Not Satisfiable"),
            StatusCode::ExpectationFailed => Some("Expectation Failed"),
            StatusCode::MisdirectedRequest => Some("Misdirected Request"),
            StatusCode::UnprocessableEntity => Some("Unprocessable Entity"),
            StatusCode::Locked => Some("Locked"),
            StatusCode::FailedDependency => Some("Failed Dependency"),
            StatusCode::UpgradeRequired => Some("Upgrade Required"),
            StatusCode::PreconditionRequired => Some("Precondition Required"),
            StatusCode::TooManyRequests => Some("Too Many Requests"),
            StatusCode::RequestHeaderFieldsTooLarge => Some("Request Header Fields Too Large"),
            StatusCode::UnavailableForLegalReasons => Some("Unavailable For Legal Reasons"),
            StatusCode::InternalServerError => Some("Internal Server Error"),
            StatusCode::NotImplemented => Some("Not Implemented"),
            StatusCode::BadGateway => Some("Bad Gateway"),
            StatusCode::ServiceUnavailable => Some("Service Unavailable"),
            StatusCode::GatewayTimeout => Some("Gateway Timeout"),
            StatusCode::HttpVersionNotSupported => Some("HTTP Version Not Supported"),
            StatusCode::VariantAlsoNegotiates => Some("Variant Also Negotiates"),
            StatusCode::InsufficientStorage => Some("Insufficient Storage"),
            StatusCode::LoopDetected => Some("Loop Detected"),
            StatusCode::NotExtended => Some("Not Extended"),
            StatusCode::NetworkAuthenticationRequired => Some("Network Authentication Required"),
            _ => None
        }
    }

    pub fn is_informational(&self) -> bool {
        self.class() == StatusClass::Informational
    }

    pub fn is_success(&self) -> bool {
        self.class() == StatusClass::Success
    }

    pub fn is_redirection(&self) -> bool {
        self.class() == StatusClass::Redirection
    }

    pub fn is_client_error(&self) -> bool {
        self.class() == StatusClass::ClientError
    }

    pub fn is_server_error(&self) -> bool {
        self.class() == StatusClass::ServerError
    }

    pub fn is_error(&self) -> bool {
        self.is_client_error() || self.is_server_error()
    }
}

impl Into<u16> for StatusCode {
    fn into(self) -> u16 {
        self.to_u16()
    }
}

impl From<u16> for StatusCode {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unregistered_status() {
        assert!(StatusCode::Unregistered(612).reason_phrase().is_none());
        assert_eq!(StatusClass::None, StatusCode::Unregistered(612).class());
    }

    #[test]
    fn test_registered_statuses() {
        test_status_code(StatusCode::Continue, 100, "Continue", StatusClass::Informational);
        test_status_code(StatusCode::SwitchingProtocols, 101, "Switching Protocols", StatusClass::Informational);
        test_status_code(StatusCode::Processing, 102, "Processing", StatusClass::Informational);
        test_status_code(StatusCode::Ok, 200, "OK", StatusClass::Success);
        test_status_code(StatusCode::Created, 201, "Created", StatusClass::Success);
        test_status_code(StatusCode::Accepted, 202, "Accepted", StatusClass::Success);
        test_status_code(StatusCode::NonAuthoritativeInformation, 203, "Non-Authoritative Information", StatusClass::Success);
        test_status_code(StatusCode::NoContent, 204, "No Content", StatusClass::Success);
        test_status_code(StatusCode::ResetContent, 205, "Reset Content", StatusClass::Success);
        test_status_code(StatusCode::PartialContent, 206, "Partial Content", StatusClass::Success);
        test_status_code(StatusCode::MultiStatus, 207, "Multi-Status", StatusClass::Success);
        test_status_code(StatusCode::AlreadyReported, 208, "Already Reported", StatusClass::Success);
        test_status_code(StatusCode::ImUsed, 226, "IM Used", StatusClass::Success);
        test_status_code(StatusCode::MultipleChoices, 300, "Multiple Choices", StatusClass::Redirection);
        test_status_code(StatusCode::MovedPermanently, 301, "Moved Permanently", StatusClass::Redirection);
        test_status_code(StatusCode::Found, 302, "Found", StatusClass::Redirection);
        test_status_code(StatusCode::SeeOther, 303, "See Other", StatusClass::Redirection);
        test_status_code(StatusCode::NotModified, 304, "Not Modified", StatusClass::Redirection);
        test_status_code(StatusCode::UseProxy, 305, "Use Proxy", StatusClass::Redirection);
        test_status_code(StatusCode::TemporaryRedirect, 307, "Temporary Redirect", StatusClass::Redirection);
        test_status_code(StatusCode::PermanentRedirect, 308, "Permanent Redirect", StatusClass::Redirection);
        test_status_code(StatusCode::BadRequest, 400, "Bad Request", StatusClass::ClientError);
        test_status_code(StatusCode::Unauthorized, 401, "Unauthorized", StatusClass::ClientError);
        test_status_code(StatusCode::PaymentRequired, 402, "Payment Required", StatusClass::ClientError);
        test_status_code(StatusCode::Forbidden, 403, "Forbidden", StatusClass::ClientError);
        test_status_code(StatusCode::NotFound, 404, "Not Found", StatusClass::ClientError);
        test_status_code(StatusCode::MethodNotAllowed, 405, "Method Not Allowed", StatusClass::ClientError);
        test_status_code(StatusCode::NotAcceptable, 406, "Not Acceptable", StatusClass::ClientError);
        test_status_code(StatusCode::ProxyAuthenticationRequired, 407, "Proxy Authentication Required", StatusClass::ClientError);
        test_status_code(StatusCode::RequestTimeout, 408, "Request Timeout", StatusClass::ClientError);
        test_status_code(StatusCode::Conflict, 409, "Conflict", StatusClass::ClientError);
        test_status_code(StatusCode::Gone, 410, "Gone", StatusClass::ClientError);
        test_status_code(StatusCode::LengthRequired, 411, "Length Required", StatusClass::ClientError);
        test_status_code(StatusCode::PreconditionFailed, 412, "Precondition Failed", StatusClass::ClientError);
        test_status_code(StatusCode::PayloadTooLarge, 413, "Payload Too Large", StatusClass::ClientError);
        test_status_code(StatusCode::UriTooLong, 414, "URI Too Long", StatusClass::ClientError);
        test_status_code(StatusCode::UnsupportedMediaType, 415, "Unsupported Media Type", StatusClass::ClientError);
        test_status_code(StatusCode::RangeNotSatisfiable, 416, "Range Not Satisfiable", StatusClass::ClientError);
        test_status_code(StatusCode::ExpectationFailed, 417, "Expectation Failed", StatusClass::ClientError);
        test_status_code(StatusCode::MisdirectedRequest, 421, "Misdirected Request", StatusClass::ClientError);
        test_status_code(StatusCode::UnprocessableEntity, 422, "Unprocessable Entity", StatusClass::ClientError);
        test_status_code(StatusCode::Locked, 423, "Locked", StatusClass::ClientError);
        test_status_code(StatusCode::FailedDependency, 424, "Failed Dependency", StatusClass::ClientError);
        test_status_code(StatusCode::UpgradeRequired, 426, "Upgrade Required", StatusClass::ClientError);
        test_status_code(StatusCode::PreconditionRequired, 428, "Precondition Required", StatusClass::ClientError);
        test_status_code(StatusCode::TooManyRequests, 429, "Too Many Requests", StatusClass::ClientError);
        test_status_code(StatusCode::RequestHeaderFieldsTooLarge, 431, "Request Header Fields Too Large", StatusClass::ClientError);
        test_status_code(StatusCode::UnavailableForLegalReasons, 451, "Unavailable For Legal Reasons", StatusClass::ClientError);
        test_status_code(StatusCode::InternalServerError, 500, "Internal Server Error", StatusClass::ServerError);
        test_status_code(StatusCode::NotImplemented, 501, "Not Implemented", StatusClass::ServerError);
        test_status_code(StatusCode::BadGateway, 502, "Bad Gateway", StatusClass::ServerError);
        test_status_code(StatusCode::ServiceUnavailable, 503, "Service Unavailable", StatusClass::ServerError);
        test_status_code(StatusCode::GatewayTimeout, 504, "Gateway Timeout", StatusClass::ServerError);
        test_status_code(StatusCode::HttpVersionNotSupported, 505, "HTTP Version Not Supported", StatusClass::ServerError);
        test_status_code(StatusCode::VariantAlsoNegotiates, 506, "Variant Also Negotiates", StatusClass::ServerError);
        test_status_code(StatusCode::InsufficientStorage, 507, "Insufficient Storage", StatusClass::ServerError);
        test_status_code(StatusCode::LoopDetected, 508, "Loop Detected", StatusClass::ServerError);
        test_status_code(StatusCode::NotExtended, 510, "Not Extended", StatusClass::ServerError);
        test_status_code(StatusCode::NetworkAuthenticationRequired, 511, "Network Authentication Required", StatusClass::ServerError);
    }

    fn test_status_code(status: StatusCode, value: u16, reason_phrase: &str, class: StatusClass) {
        assert_eq!(status, value.into());
        assert_eq!(status, StatusCode::from_u16(value));
        assert_eq!(status, StatusCode::from(value));
        assert_eq!(value, status.into());
        assert_eq!(value, status.to_u16());
        assert_eq!(class, status.class());
        assert_eq!(reason_phrase, status.canonical_reason().unwrap());
    }
}
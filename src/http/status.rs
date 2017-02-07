#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Status {
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

impl Status {
    pub fn from_u16(value: u16) -> Self {
        match value {
            100 => Status::Continue,
            101 => Status::SwitchingProtocols,
            102 => Status::Processing,
            200 => Status::Ok,
            201 => Status::Created,
            202 => Status::Accepted,
            203 => Status::NonAuthoritativeInformation,
            204 => Status::NoContent,
            205 => Status::ResetContent,
            206 => Status::PartialContent,
            207 => Status::MultiStatus,
            208 => Status::AlreadyReported,
            226 => Status::ImUsed,
            300 => Status::MultipleChoices,
            301 => Status::MovedPermanently,
            302 => Status::Found,
            303 => Status::SeeOther,
            304 => Status::NotModified,
            305 => Status::UseProxy,
            307 => Status::TemporaryRedirect,
            308 => Status::PermanentRedirect,
            400 => Status::BadRequest,
            401 => Status::Unauthorized,
            402 => Status::PaymentRequired,
            403 => Status::Forbidden,
            404 => Status::NotFound,
            405 => Status::MethodNotAllowed,
            406 => Status::NotAcceptable,
            407 => Status::ProxyAuthenticationRequired,
            408 => Status::RequestTimeout,
            409 => Status::Conflict,
            410 => Status::Gone,
            411 => Status::LengthRequired,
            412 => Status::PreconditionFailed,
            413 => Status::PayloadTooLarge,
            414 => Status::UriTooLong,
            415 => Status::UnsupportedMediaType,
            416 => Status::RangeNotSatisfiable,
            417 => Status::ExpectationFailed,
            421 => Status::MisdirectedRequest,
            422 => Status::UnprocessableEntity,
            423 => Status::Locked,
            424 => Status::FailedDependency,
            426 => Status::UpgradeRequired,
            428 => Status::PreconditionRequired,
            429 => Status::TooManyRequests,
            431 => Status::RequestHeaderFieldsTooLarge,
            451 => Status::UnavailableForLegalReasons,
            500 => Status::InternalServerError,
            501 => Status::NotImplemented,
            502 => Status::BadGateway,
            503 => Status::ServiceUnavailable,
            504 => Status::GatewayTimeout,
            505 => Status::HttpVersionNotSupported,
            506 => Status::VariantAlsoNegotiates,
            507 => Status::InsufficientStorage,
            508 => Status::LoopDetected,
            510 => Status::NotExtended,
            511 => Status::NetworkAuthenticationRequired,
            _ => Status::Unregistered(value)
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            Status::Continue => 100,
            Status::SwitchingProtocols => 101,
            Status::Processing => 102,
            Status::Ok => 200,
            Status::Created => 201,
            Status::Accepted => 202,
            Status::NonAuthoritativeInformation => 203,
            Status::NoContent => 204,
            Status::ResetContent => 205,
            Status::PartialContent => 206,
            Status::MultiStatus => 207,
            Status::AlreadyReported => 208,
            Status::ImUsed => 226,
            Status::MultipleChoices => 300,
            Status::MovedPermanently => 301,
            Status::Found => 302,
            Status::SeeOther => 303,
            Status::NotModified => 304,
            Status::UseProxy => 305,
            Status::TemporaryRedirect => 307,
            Status::PermanentRedirect => 308,
            Status::BadRequest => 400,
            Status::Unauthorized => 401,
            Status::PaymentRequired => 402,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::MethodNotAllowed => 405,
            Status::NotAcceptable => 406,
            Status::ProxyAuthenticationRequired => 407,
            Status::RequestTimeout => 408,
            Status::Conflict => 409,
            Status::Gone => 410,
            Status::LengthRequired => 411,
            Status::PreconditionFailed => 412,
            Status::PayloadTooLarge => 413,
            Status::UriTooLong => 414,
            Status::UnsupportedMediaType => 415,
            Status::RangeNotSatisfiable => 416,
            Status::ExpectationFailed => 417,
            Status::MisdirectedRequest => 421,
            Status::UnprocessableEntity => 422,
            Status::Locked => 423,
            Status::FailedDependency => 424,
            Status::UpgradeRequired => 426,
            Status::PreconditionRequired => 428,
            Status::TooManyRequests => 429,
            Status::RequestHeaderFieldsTooLarge => 431,
            Status::UnavailableForLegalReasons => 451,
            Status::InternalServerError => 500,
            Status::NotImplemented => 501,
            Status::BadGateway => 502,
            Status::ServiceUnavailable => 503,
            Status::GatewayTimeout => 504,
            Status::HttpVersionNotSupported => 505,
            Status::VariantAlsoNegotiates => 506,
            Status::InsufficientStorage => 507,
            Status::LoopDetected => 508,
            Status::NotExtended => 510,
            Status::NetworkAuthenticationRequired => 511,
            Status::Unregistered(value) => value
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

    pub fn reason_phrase(&self) -> Option<&'static str> {
        match *self {
            Status::Continue => Some("Continue"),
            Status::SwitchingProtocols => Some("Switching Protocols"),
            Status::Processing => Some("Processing"),
            Status::Ok => Some("OK"),
            Status::Created => Some("Created"),
            Status::Accepted => Some("Accepted"),
            Status::NonAuthoritativeInformation => Some("Non-Authoritative Information"),
            Status::NoContent => Some("No Content"),
            Status::ResetContent => Some("Reset Content"),
            Status::PartialContent => Some("Partial Content"),
            Status::MultiStatus => Some("Multi-Status"),
            Status::AlreadyReported => Some("Already Reported"),
            Status::ImUsed => Some("IM Used"),
            Status::MultipleChoices => Some("Multiple Choices"),
            Status::MovedPermanently => Some("Moved Permanently"),
            Status::Found => Some("Found"),
            Status::SeeOther => Some("See Other"),
            Status::NotModified => Some("Not Modified"),
            Status::UseProxy => Some("Use Proxy"),
            Status::TemporaryRedirect => Some("Temporary Redirect"),
            Status::PermanentRedirect => Some("Permanent Redirect"),
            Status::BadRequest => Some("Bad Request"),
            Status::Unauthorized => Some("Unauthorized"),
            Status::PaymentRequired => Some("Payment Required"),
            Status::Forbidden => Some("Forbidden"),
            Status::NotFound => Some("Not Found"),
            Status::MethodNotAllowed => Some("Method Not Allowed"),
            Status::NotAcceptable => Some("Not Acceptable"),
            Status::ProxyAuthenticationRequired => Some("Proxy Authentication Required"),
            Status::RequestTimeout => Some("Request Timeout"),
            Status::Conflict => Some("Conflict"),
            Status::Gone => Some("Gone"),
            Status::LengthRequired => Some("Length Required"),
            Status::PreconditionFailed => Some("Precondition Failed"),
            Status::PayloadTooLarge => Some("Payload Too Large"),
            Status::UriTooLong => Some("URI Too Long"),
            Status::UnsupportedMediaType => Some("Unsupported Media Type"),
            Status::RangeNotSatisfiable => Some("Range Not Satisfiable"),
            Status::ExpectationFailed => Some("Expectation Failed"),
            Status::MisdirectedRequest => Some("Misdirected Request"),
            Status::UnprocessableEntity => Some("Unprocessable Entity"),
            Status::Locked => Some("Locked"),
            Status::FailedDependency => Some("Failed Dependency"),
            Status::UpgradeRequired => Some("Upgrade Required"),
            Status::PreconditionRequired => Some("Precondition Required"),
            Status::TooManyRequests => Some("Too Many Requests"),
            Status::RequestHeaderFieldsTooLarge => Some("Request Header Fields Too Large"),
            Status::UnavailableForLegalReasons => Some("Unavailable For Legal Reasons"),
            Status::InternalServerError => Some("Internal Server Error"),
            Status::NotImplemented => Some("Not Implemented"),
            Status::BadGateway => Some("Bad Gateway"),
            Status::ServiceUnavailable => Some("Service Unavailable"),
            Status::GatewayTimeout => Some("Gateway Timeout"),
            Status::HttpVersionNotSupported => Some("HTTP Version Not Supported"),
            Status::VariantAlsoNegotiates => Some("Variant Also Negotiates"),
            Status::InsufficientStorage => Some("Insufficient Storage"),
            Status::LoopDetected => Some("Loop Detected"),
            Status::NotExtended => Some("Not Extended"),
            Status::NetworkAuthenticationRequired => Some("Network Authentication Required"),
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

impl Into<u16> for Status {
    fn into(self) -> u16 {
        self.to_u16()
    }
}

impl From<u16> for Status {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unregistered_status() {
        assert!(Status::Unregistered(612).reason_phrase().is_none());
        assert_eq!(StatusClass::None, Status::Unregistered(612).class());
    }

    #[test]
    fn test_registered_statuses() {
        test_status_code(Status::Continue, 100, "Continue", StatusClass::Informational);
        test_status_code(Status::SwitchingProtocols, 101, "Switching Protocols", StatusClass::Informational);
        test_status_code(Status::Processing, 102, "Processing", StatusClass::Informational);
        test_status_code(Status::Ok, 200, "OK", StatusClass::Success);
        test_status_code(Status::Created, 201, "Created", StatusClass::Success);
        test_status_code(Status::Accepted, 202, "Accepted", StatusClass::Success);
        test_status_code(Status::NonAuthoritativeInformation, 203, "Non-Authoritative Information", StatusClass::Success);
        test_status_code(Status::NoContent, 204, "No Content", StatusClass::Success);
        test_status_code(Status::ResetContent, 205, "Reset Content", StatusClass::Success);
        test_status_code(Status::PartialContent, 206, "Partial Content", StatusClass::Success);
        test_status_code(Status::MultiStatus, 207, "Multi-Status", StatusClass::Success);
        test_status_code(Status::AlreadyReported, 208, "Already Reported", StatusClass::Success);
        test_status_code(Status::ImUsed, 226, "IM Used", StatusClass::Success);
        test_status_code(Status::MultipleChoices, 300, "Multiple Choices", StatusClass::Redirection);
        test_status_code(Status::MovedPermanently, 301, "Moved Permanently", StatusClass::Redirection);
        test_status_code(Status::Found, 302, "Found", StatusClass::Redirection);
        test_status_code(Status::SeeOther, 303, "See Other", StatusClass::Redirection);
        test_status_code(Status::NotModified, 304, "Not Modified", StatusClass::Redirection);
        test_status_code(Status::UseProxy, 305, "Use Proxy", StatusClass::Redirection);
        test_status_code(Status::TemporaryRedirect, 307, "Temporary Redirect", StatusClass::Redirection);
        test_status_code(Status::PermanentRedirect, 308, "Permanent Redirect", StatusClass::Redirection);
        test_status_code(Status::BadRequest, 400, "Bad Request", StatusClass::ClientError);
        test_status_code(Status::Unauthorized, 401, "Unauthorized", StatusClass::ClientError);
        test_status_code(Status::PaymentRequired, 402, "Payment Required", StatusClass::ClientError);
        test_status_code(Status::Forbidden, 403, "Forbidden", StatusClass::ClientError);
        test_status_code(Status::NotFound, 404, "Not Found", StatusClass::ClientError);
        test_status_code(Status::MethodNotAllowed, 405, "Method Not Allowed", StatusClass::ClientError);
        test_status_code(Status::NotAcceptable, 406, "Not Acceptable", StatusClass::ClientError);
        test_status_code(Status::ProxyAuthenticationRequired, 407, "Proxy Authentication Required", StatusClass::ClientError);
        test_status_code(Status::RequestTimeout, 408, "Request Timeout", StatusClass::ClientError);
        test_status_code(Status::Conflict, 409, "Conflict", StatusClass::ClientError);
        test_status_code(Status::Gone, 410, "Gone", StatusClass::ClientError);
        test_status_code(Status::LengthRequired, 411, "Length Required", StatusClass::ClientError);
        test_status_code(Status::PreconditionFailed, 412, "Precondition Failed", StatusClass::ClientError);
        test_status_code(Status::PayloadTooLarge, 413, "Payload Too Large", StatusClass::ClientError);
        test_status_code(Status::UriTooLong, 414, "URI Too Long", StatusClass::ClientError);
        test_status_code(Status::UnsupportedMediaType, 415, "Unsupported Media Type", StatusClass::ClientError);
        test_status_code(Status::RangeNotSatisfiable, 416, "Range Not Satisfiable", StatusClass::ClientError);
        test_status_code(Status::ExpectationFailed, 417, "Expectation Failed", StatusClass::ClientError);
        test_status_code(Status::MisdirectedRequest, 421, "Misdirected Request", StatusClass::ClientError);
        test_status_code(Status::UnprocessableEntity, 422, "Unprocessable Entity", StatusClass::ClientError);
        test_status_code(Status::Locked, 423, "Locked", StatusClass::ClientError);
        test_status_code(Status::FailedDependency, 424, "Failed Dependency", StatusClass::ClientError);
        test_status_code(Status::UpgradeRequired, 426, "Upgrade Required", StatusClass::ClientError);
        test_status_code(Status::PreconditionRequired, 428, "Precondition Required", StatusClass::ClientError);
        test_status_code(Status::TooManyRequests, 429, "Too Many Requests", StatusClass::ClientError);
        test_status_code(Status::RequestHeaderFieldsTooLarge, 431, "Request Header Fields Too Large", StatusClass::ClientError);
        test_status_code(Status::UnavailableForLegalReasons, 451, "Unavailable For Legal Reasons", StatusClass::ClientError);
        test_status_code(Status::InternalServerError, 500, "Internal Server Error", StatusClass::ServerError);
        test_status_code(Status::NotImplemented, 501, "Not Implemented", StatusClass::ServerError);
        test_status_code(Status::BadGateway, 502, "Bad Gateway", StatusClass::ServerError);
        test_status_code(Status::ServiceUnavailable, 503, "Service Unavailable", StatusClass::ServerError);
        test_status_code(Status::GatewayTimeout, 504, "Gateway Timeout", StatusClass::ServerError);
        test_status_code(Status::HttpVersionNotSupported, 505, "HTTP Version Not Supported", StatusClass::ServerError);
        test_status_code(Status::VariantAlsoNegotiates, 506, "Variant Also Negotiates", StatusClass::ServerError);
        test_status_code(Status::InsufficientStorage, 507, "Insufficient Storage", StatusClass::ServerError);
        test_status_code(Status::LoopDetected, 508, "Loop Detected", StatusClass::ServerError);
        test_status_code(Status::NotExtended, 510, "Not Extended", StatusClass::ServerError);
        test_status_code(Status::NetworkAuthenticationRequired, 511, "Network Authentication Required", StatusClass::ServerError);
    }

    fn test_status_code(status: Status, value: u16, reason_phrase: &str, class: StatusClass) {
        assert_eq!(status, value.into());
        assert_eq!(status, Status::from_u16(value));
        assert_eq!(status, Status::from(value));
        assert_eq!(value, status.into());
        assert_eq!(value, status.to_u16());
        assert_eq!(class, status.class());
        assert_eq!(reason_phrase, status.reason_phrase().unwrap());
    }
}
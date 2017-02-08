var searchIndex = {};
searchIndex["teapot"] = {"doc":"","items":[[0,"mime","teapot","",null,null],[8,"Mime","teapot::mime","",null,null],[10,"top","","",0,{"inputs":[],"output":{"name":"str"}}],[10,"sub","","",0,{"inputs":[],"output":{"name":"str"}}],[0,"http","teapot","",null,null],[0,"headers","teapot::http","",null,null],[3,"RawHeader","teapot::http::headers","",null,null],[3,"Headers","","Examples",null,null],[3,"DntHeader","","",null,null],[3,"UserAgentHeader","","",null,null],[4,"Dnt","","",null,null],[13,"Disabled","","",1,null],[13,"Enabled","","",1,null],[13,"Unspecified","","",1,null],[8,"TypedHeader","","Examples",null,null],[10,"name","","This is the name of the header in lower case. It is used in [`Headers`] to look up the raw header(s).",2,{"inputs":[],"output":{"name":"str"}}],[10,"canonical_name","","This is the name of the header in its canonical form. Used by [`to_raw`] as the header name.",2,{"inputs":[],"output":{"name":"str"}}],[10,"parse","","Converts a list of raw values to a `TypedHeader` The list is required for headers like `Set-Cookie` which might appear multiple times in a response. Other headers might only use the first value of `raw` and ignore the rest.",2,null],[10,"raw_values","","Returns the raw values of this header. Used by [`to_raw`] as the header value. When multiple values are returned, [`to_raw`] will return multiple raw headers for each value.",2,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"to_raw","","Converts the header back to one or more [`RawHeader`]s",2,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"cmp","","",3,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",3,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":{"name":"option"}}],[11,"lt","","",3,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":{"name":"bool"}}],[11,"le","","",3,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":{"name":"bool"}}],[11,"gt","","",3,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":{"name":"bool"}}],[11,"ge","","",3,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":{"name":"bool"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":{"name":"bool"}}],[11,"new","","",3,{"inputs":[{"name":"s"},{"name":"s"}],"output":{"name":"self"}}],[11,"parse","","",3,{"inputs":[{"name":"s"}],"output":{"name":"self"}}],[11,"lower_name","","",3,{"inputs":[{"name":"self"}],"output":{"name":"string"}}],[11,"name","","",3,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"value","","",3,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"new","","",4,{"inputs":[],"output":{"name":"self"}}],[11,"append","","",4,{"inputs":[{"name":"self"},{"name":"h"}],"output":null}],[11,"append_raw","","",4,{"inputs":[{"name":"self"},{"name":"rawheader"}],"output":null}],[11,"get","","",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"get_raw","","",4,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"vec"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"dnt"}],"output":{"name":"bool"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"dnt"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",5,{"inputs":[{"name":"self"},{"name":"dntheader"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"self"},{"name":"dntheader"}],"output":{"name":"bool"}}],[11,"new","","",5,{"inputs":[{"name":"dnt"}],"output":{"name":"self"}}],[11,"value","","",5,{"inputs":[{"name":"self"}],"output":{"name":"dnt"}}],[11,"name","","",5,{"inputs":[],"output":{"name":"str"}}],[11,"canonical_name","","",5,{"inputs":[],"output":{"name":"str"}}],[11,"parse","","",5,null],[11,"raw_values","","",5,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"useragentheader"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"self"},{"name":"useragentheader"}],"output":{"name":"bool"}}],[11,"value","","",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"name","","",6,{"inputs":[],"output":{"name":"str"}}],[11,"canonical_name","","",6,{"inputs":[],"output":{"name":"str"}}],[11,"parse","","",6,null],[11,"raw_values","","",6,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[0,"message","teapot::http","",null,null],[3,"Message","teapot::http::message","",null,null],[4,"ParseError","","",null,null],[13,"Hello","","",7,null],[13,"FromUtf8Error","","",7,null],[13,"ReadError","","",7,null],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",7,{"inputs":[{"name":"fromutf8error"}],"output":{"name":"self"}}],[11,"from","","",7,{"inputs":[{"name":"lineserror"}],"output":{"name":"self"}}],[11,"start_line","","",8,{"inputs":[{"name":"self"}],"output":{"name":"string"}}],[11,"new","","",8,{"inputs":[{"name":"s"},{"name":"headers"},{"name":"read"}],"output":{"name":"self"}}],[11,"parse","","",8,{"inputs":[{"name":"read"}],"output":{"name":"result"}}],[0,"status","teapot::http","",null,null],[4,"StatusCode","teapot::http::status","",null,null],[13,"Continue","","100 Continue RFC7231, Section 6.2.1",9,null],[13,"SwitchingProtocols","","101 Switching Protocols RFC7231, Section 6.2.2",9,null],[13,"Processing","","102 Processing RFC2518",9,null],[13,"Ok","","200 OK RFC7231, Section 6.3.1",9,null],[13,"Created","","201 Created RFC7231, Section 6.3.2",9,null],[13,"Accepted","","202 Accepted RFC7231, Section 6.3.3",9,null],[13,"NonAuthoritativeInformation","","203 Non-Authoritative Information RFC7231, Section 6.3.4",9,null],[13,"NoContent","","204 No Content RFC7231, Section 6.3.5",9,null],[13,"ResetContent","","205 Reset Content RFC7231, Section 6.3.6",9,null],[13,"PartialContent","","206 Partial Content RFC7233, Section 4.1",9,null],[13,"MultiStatus","","207 Multi-Status RFC4918",9,null],[13,"AlreadyReported","","208 Already Reported RFC5842",9,null],[13,"ImUsed","","226 IM Used RFC3229",9,null],[13,"MultipleChoices","","300 Multiple Choices RFC7231, Section 6.4.1",9,null],[13,"MovedPermanently","","301 Moved Permanently RFC7231, Section 6.4.2",9,null],[13,"Found","","302 Found RFC7231, Section 6.4.3",9,null],[13,"SeeOther","","303 See Other RFC7231, Section 6.4.4",9,null],[13,"NotModified","","304 Not Modified RFC7232, Section 4.1",9,null],[13,"UseProxy","","305 Use Proxy RFC7231, Section 6.4.5",9,null],[13,"TemporaryRedirect","","307 Temporary Redirect RFC7231, Section 6.4.7",9,null],[13,"PermanentRedirect","","308 Permanent Redirect RFC7538",9,null],[13,"BadRequest","","400 Bad Request RFC7231, Section 6.5.1",9,null],[13,"Unauthorized","","401 Unauthorized RFC7235, Section 3.1",9,null],[13,"PaymentRequired","","402 Payment Required RFC7231, Section 6.5.2",9,null],[13,"Forbidden","","403 Forbidden RFC7231, Section 6.5.3",9,null],[13,"NotFound","","404 Not Found RFC7231, Section 6.5.4",9,null],[13,"MethodNotAllowed","","405 Method Not Allowed RFC7231, Section 6.5.5",9,null],[13,"NotAcceptable","","406 Not Acceptable RFC7231, Section 6.5.6",9,null],[13,"ProxyAuthenticationRequired","","407 Proxy Authentication Required RFC7235, Section 3.2",9,null],[13,"RequestTimeout","","408 Request Timeout RFC7231, Section 6.5.7",9,null],[13,"Conflict","","409 Conflict RFC7231, Section 6.5.8",9,null],[13,"Gone","","410 Gone RFC7231, Section 6.5.9",9,null],[13,"LengthRequired","","411 Length Required RFC7231, Section 6.5.10",9,null],[13,"PreconditionFailed","","412 Precondition Failed RFC7232, Section 4.2",9,null],[13,"PayloadTooLarge","","413 Payload Too Large RFC7231, Section 6.5.11",9,null],[13,"UriTooLong","","414 URI Too Long RFC7231, Section 6.5.12",9,null],[13,"UnsupportedMediaType","","415 Unsupported Media Type [RFC7231, Section 6.5.13]RFC7694, Section 3",9,null],[13,"RangeNotSatisfiable","","416 Range Not Satisfiable RFC7233, Section 4.4",9,null],[13,"ExpectationFailed","","417 Expectation Failed RFC7231, Section 6.5.14",9,null],[13,"ImATeapot","","418 I'm a Teapot RFC7168, Section 2.3.3",9,null],[13,"MisdirectedRequest","","421 Misdirected Request RFC7540, Section 9.1.2",9,null],[13,"UnprocessableEntity","","422 Unprocessable Entity RFC4918",9,null],[13,"Locked","","423 Locked RFC4918",9,null],[13,"FailedDependency","","424 Failed Dependency RFC4918",9,null],[13,"UpgradeRequired","","426 Upgrade Required RFC7231, Section 6.5.15",9,null],[13,"PreconditionRequired","","428 Precondition Required RFC6585",9,null],[13,"TooManyRequests","","429 Too Many Requests RFC6585",9,null],[13,"RequestHeaderFieldsTooLarge","","431 Request Header Fields Too Large RFC6585",9,null],[13,"UnavailableForLegalReasons","","451 Unavailable For Legal Reasons RFC7725",9,null],[13,"InternalServerError","","500 Internal Server Error RFC7231, Section 6.6.1",9,null],[13,"NotImplemented","","501 Not Implemented RFC7231, Section 6.6.2",9,null],[13,"BadGateway","","502 Bad Gateway RFC7231, Section 6.6.3",9,null],[13,"ServiceUnavailable","","503 Service Unavailable RFC7231, Section 6.6.4",9,null],[13,"GatewayTimeout","","504 Gateway Timeout RFC7231, Section 6.6.5",9,null],[13,"HttpVersionNotSupported","","505 HTTP Version Not Supported RFC7231, Section 6.6.6",9,null],[13,"VariantAlsoNegotiates","","506 Variant Also Negotiates RFC2295",9,null],[13,"InsufficientStorage","","507 Insufficient Storage RFC4918",9,null],[13,"LoopDetected","","508 Loop Detected RFC5842",9,null],[13,"NotExtended","","510 Not Extended RFC2774",9,null],[13,"NetworkAuthenticationRequired","","511 Network Authentication Required RFC6585",9,null],[13,"Unregistered","","Catch-all for unregistered status codes",9,null],[4,"StatusClass","","",null,null],[13,"Informational","","1xx: Informational - Request received, continuing process",10,null],[13,"Success","","2xx: Success - The action was successfully received, understood, and accepted",10,null],[13,"Redirection","","3xx: Redirection - Further action must be taken in order to complete the request",10,null],[13,"ClientError","","4xx: Client Error - The request contains bad syntax or cannot be fulfilled",10,null],[13,"ServerError","","5xx: Server Error - The server failed to fulfill an apparently valid request",10,null],[13,"None","","Catch-All for everything outside the defined classes",10,null],[11,"clone","","",9,{"inputs":[{"name":"self"}],"output":{"name":"statuscode"}}],[11,"fmt","","",9,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",9,{"inputs":[{"name":"self"},{"name":"statuscode"}],"output":{"name":"bool"}}],[11,"ne","","",9,{"inputs":[{"name":"self"},{"name":"statuscode"}],"output":{"name":"bool"}}],[11,"clone","","",10,{"inputs":[{"name":"self"}],"output":{"name":"statusclass"}}],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",10,{"inputs":[{"name":"self"},{"name":"statusclass"}],"output":{"name":"bool"}}],[11,"from_u16","","",9,{"inputs":[{"name":"u16"}],"output":{"name":"self"}}],[11,"to_u16","","",9,{"inputs":[{"name":"self"}],"output":{"name":"u16"}}],[11,"class","","",9,{"inputs":[{"name":"self"}],"output":{"name":"statusclass"}}],[11,"canonical_reason","","",9,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"is_informational","","",9,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_success","","",9,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_redirection","","",9,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_client_error","","",9,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_server_error","","",9,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"is_error","","",9,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"into","","",9,{"inputs":[{"name":"self"}],"output":{"name":"u16"}}],[11,"from","","",9,{"inputs":[{"name":"u16"}],"output":{"name":"self"}}],[11,"fmt","","",9,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[14,"try_to_opt","teapot","",null,null]],"paths":[[8,"Mime"],[4,"Dnt"],[8,"TypedHeader"],[3,"RawHeader"],[3,"Headers"],[3,"DntHeader"],[3,"UserAgentHeader"],[4,"ParseError"],[3,"Message"],[4,"StatusCode"],[4,"StatusClass"]]};
initSearch(searchIndex);

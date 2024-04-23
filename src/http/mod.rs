pub mod response_codes {
    /// The request completed successfully.
    pub const OK: u16 = 200;
    /// The entity was created successfully.
    pub const CREATED: u16 = 201;
    /// The request completed successfully but returned no content.
    pub const NO_CONTENT: u16 = 204;
    /// The entity was not modified (no action was taken).
    pub const NOT_MODIFIED: u16 = 304;
    /// The request was improperly formatted, or the server couldn't understand it.
    pub const BAD_REQUEST: u16 = 400;
    /// The *Authorization* header was missing or invalid.
    pub const UNAUTHORIZED: u16 = 401;
    /// The *Authorization* token you passed did not have permission to the resource.
    pub const FORBIDDEN: u16 = 403;
    /// The resource at the location specified doesn't exist.
    pub const NOT_FOUND: u16 = 404;
    /// The HTTP method used is not valid for the location specified.
    pub const NOT_ALLOWED: u16 = 405;
    /// You are being rate limited.
    pub const RATE_LIMIT: u16 = 429;
    /// There was not a gateway available to process your request.
    /// 
    /// Wait a bit and retry.
    pub const UNAVAILABLE: u16 = 502;
}
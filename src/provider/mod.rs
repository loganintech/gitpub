pub mod bitbucket;
pub mod github;
pub mod gitlab;

/// Provider outlines the requirements for a provider
pub trait Provider {
    ///A JSON payload to send to the provider's endpoint
    fn payload(&self) -> String;
    ///The endpoint to send the POST, usually the form of api.___.com
    fn endpoint(&self) -> String;
    /// A method to extract the remote url from the response headers.
    fn extract_url(&self, _: &reqwest::header::HeaderMap) -> String;
    /// The response token formatted as the request body.
    fn token(&self) -> String;
    /// The response header key. Like `Authorization` or `Bearer`
    fn auth_header(&self) -> String;
}

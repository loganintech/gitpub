pub mod bitbucket;
pub mod github;
pub mod gitlab;

pub trait Provider {
    fn payload(&self) -> String;
    fn endpoint(&self) -> String;
    fn extract_url(&self, _: &reqwest::header::HeaderMap) -> String;
    fn token(&self) -> String;
    fn auth_header(&self) -> String;
    fn ssh_url(&self, _: &reqwest::header::HeaderMap) -> Option<String> { None }
}

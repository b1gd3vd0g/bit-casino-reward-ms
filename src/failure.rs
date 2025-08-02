pub trait Failure {
    /// Generate a user-friendly string to describe the Failure. This will be returned in the HTTP
    /// response body and should **never** include sensitive information.
    fn message(&self) -> String;
}

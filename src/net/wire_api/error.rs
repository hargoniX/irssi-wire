#[derive(Debug)]
pub enum ApiError {
    JsonParseError(Box<dyn std::error::Error>),
    HttpError(hyper::error::Error),
}

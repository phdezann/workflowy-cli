pub type GenError = Box<dyn std::error::Error>;
pub type GenResult<T> = Result<T, GenError>;

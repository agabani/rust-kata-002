#[derive(Debug)]
pub struct RustKataError {}

pub type RustKataResult<T> = Result<T, RustKataError>;

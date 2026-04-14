#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    #[error("Unexpected end of input")]
    UnexpectedEOF,
}

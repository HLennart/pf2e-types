use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("failed to parse `{structure}`, unexpected value `{field}`")]
    UnexpectedField { structure: String, field: String },
}

use crate::error::ParseError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl Size {
    pub fn try_from_short(s: &str) -> Result<Self, ParseError> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "tiny" => Ok(Self::Tiny),
            "sm" => Ok(Self::Small),
            "med" => Ok(Self::Small),
            _ => Err(ParseError::UnexpectedField {
                structure: "Size".to_string(),
                field: lower,
            }),
        }
    }
}

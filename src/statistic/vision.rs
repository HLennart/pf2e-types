use crate::error::ParseError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Vision {
    Darkvision,
    LowLightVision,
    Normal,
}

impl TryFrom<&str> for Vision {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "normal" => Ok(Self::Normal),
            "lowlightvision" => Ok(Self::LowLightVision),
            "darkvision" => Ok(Self::Darkvision),
            _ => Err(ParseError::UnexpectedField {
                structure: "Vision".to_string(),
                field: lower,
            }),
        }
    }
}

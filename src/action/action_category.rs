use serde::{Deserialize, Serialize};

use crate::error::ParseError;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum ActionCategory {
    Offensive,
    Defensive,
    Interaction,
    None,
}

impl From<ActionCategory> for Option<String> {
    fn from(value: ActionCategory) -> Self {
        use ActionCategory::*;
        match value {
            Offensive => Some("offensive".to_string()),
            Defensive => Some("defensive".to_string()),
            Interaction => Some("interaction".to_string()),
            None => std::option::Option::None,
        }
    }
}

impl TryFrom<&str> for ActionCategory {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "offensive" => Ok(Self::Offensive),
            "defensive" => Ok(Self::Defensive),
            "interaction" => Ok(Self::Interaction),
            "" => Ok(Self::None),
            _ => Err(ParseError::UnexpectedField {
                structure: "ActionCategory".to_string(),
                field: lower,
            }),
        }
    }
}
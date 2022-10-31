use crate::error::ParseError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ActionType {
    Passive,
    Action,
    Reaction,
    Free,
}

impl From<ActionType> for String {
    fn from(value: ActionType) -> Self {
        match value {
            ActionType::Action => "action".into(),
            ActionType::Free => "free".into(),
            ActionType::Passive => "passive".into(),
            ActionType::Reaction => "reaction".into(),
        }
    }
}

impl TryFrom<&str> for ActionType {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, ParseError> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "passive" => Ok(ActionType::Passive),
            "action" => Ok(ActionType::Action),
            "reaction" => Ok(ActionType::Reaction),
            "free" => Ok(ActionType::Free),
            _ => Err(ParseError::UnexpectedField {
                structure: "ActionType".to_string(),
                field: lower,
            }),
        }
    }
}

use crate::error::ParseError;

mod defense_proficiency;
mod saving_throws;
mod weapon_proficiency;

pub use defense_proficiency::DefenseProficiency;
pub use saving_throws::SavingThrows;
use serde::{Deserialize, Serialize};
pub use weapon_proficiency::WeaponProficiency;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

impl TryFrom<usize> for Proficiency {
    type Error = ParseError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use Proficiency::*;
        match value {
            0 => Ok(Untrained),
            1 => Ok(Trained),
            2 => Ok(Expert),
            3 => Ok(Master),
            4 => Ok(Legendary),
            or_else => Err(ParseError::UnexpectedField {
                structure: "Proficiency".into(),
                field: or_else.to_string(),
            }),
        }
    }
}

impl From<&Proficiency> for usize {
    fn from(value: &Proficiency) -> Self {
        use Proficiency::*;
        match value {
            Untrained => 0,
            Trained => 1,
            Expert => 2,
            Master => 3,
            Legendary => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;

    use super::Proficiency;

    #[test]
    fn parse_valid_proficiency_number() -> Result<(), ParseError> {
        let valid_proficiencies = 0..5;
        for proficiency in valid_proficiencies {
            Proficiency::try_from(proficiency)?;
        }

        Ok(())
    }

    #[test]
    fn reject_invalid_proficiency_number() {
        let invalid_proficiency = 5;

        let result = Proficiency::try_from(invalid_proficiency);

        assert!(result.is_err())
    }
}

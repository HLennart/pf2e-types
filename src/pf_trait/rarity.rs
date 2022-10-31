use serde::{Deserialize, Serialize};

use crate::error::ParseError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

impl From<Rarity> for String {
    fn from(value: Rarity) -> Self {
        match value {
            Rarity::Common => "common".into(),
            Rarity::Uncommon => "uncommon".into(),
            Rarity::Rare => "rare".into(),
            Rarity::Unique => "unique".into(),
        }
    }
}

impl TryFrom<&str> for Rarity {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "common" => Ok(Self::Common),
            "uncommon" => Ok(Self::Uncommon),
            "rare" => Ok(Self::Rare),
            "unique" => Ok(Self::Unique),
            _ => Err(ParseError::UnexpectedField {
                structure: "Rarity".to_string(),
                field: lower,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;

    use super::Rarity;

    #[test]
    fn parse_valid_rarities() -> Result<(), ParseError> {
        assert_eq!(Rarity::try_from("common")?, Rarity::Common);
        assert_eq!(Rarity::try_from("uncommon")?, Rarity::Uncommon);
        assert_eq!(Rarity::try_from("rare")?, Rarity::Rare);

        Ok(())
    }

    #[test]
    fn parse_valid_rarity_should_disregard_case() -> Result<(), ParseError> {
        assert_eq!(Rarity::try_from("CoMmoN")?, Rarity::Common);
        assert_eq!(Rarity::try_from("uNcoMMon")?, Rarity::Uncommon);
        assert_eq!(Rarity::try_from("RarE")?, Rarity::Rare);

        Ok(())
    }

    #[test]
    fn parse_rarity_reject_invalid() {
        assert!(Rarity::try_from("doesn't exist").is_err())
    }

    #[test]
    fn parse_hold_up_identity() -> Result<(), ParseError> {
        assert_eq!(
            Rarity::try_from(
                String::from(Rarity::try_from(
                    String::from(Rarity::try_from("common")?).as_str()
                )?)
                .as_str()
            )?,
            Rarity::Common
        );

        Ok(())
    }
}

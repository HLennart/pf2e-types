use crate::error::ParseError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Ability {
    pub fn try_from_string(s: &str) -> Result<Self, ParseError> {
        use Ability::*;
        let lower = s.to_lowercase();
        match lower.as_str() {
            "strength" => Ok(Strength),
            "dexterity" => Ok(Dexterity),
            "constitution" => Ok(Constitution),
            "intelligence" => Ok(Intelligence),
            "wisdom" => Ok(Wisdom),
            "charisma" => Ok(Charisma),
            _ => Err(ParseError::UnexpectedField {
                structure: "Ability".into(),
                field: lower,
            }),
        }
    }

    pub fn to_string(&self) -> String {
        use Ability::*;
        match self {
            Strength => "strength".into(),
            Dexterity => "dexterity".into(),
            Constitution => "constitution".into(),
            Intelligence => "intelligence".into(),
            Wisdom => "wisdom".into(),
            Charisma => "charisma".into(),
        }
    }

    pub fn try_from_short(s: &str) -> Result<Self, ParseError> {
        use Ability::*;
        let lower = s.to_lowercase();
        match lower.as_str() {
            "str" => Ok(Strength),
            "dex" => Ok(Dexterity),
            "con" => Ok(Constitution),
            "int" => Ok(Intelligence),
            "wis" => Ok(Wisdom),
            "cha" => Ok(Charisma),
            _ => Err(ParseError::UnexpectedField {
                structure: "Skill".into(),
                field: lower,
            }),
        }
    }

    pub fn to_short(&self) -> String {
        use Ability::*;
        match self {
            Strength => "str".into(),
            Dexterity => "dex".into(),
            Constitution => "con".into(),
            Intelligence => "int".into(),
            Wisdom => "wis".into(),
            Charisma => "cha".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::ParseError, statistic::Ability};

    #[test]
    fn parse_valid_short_abilities() -> Result<(), ParseError> {
        assert_eq!(Ability::Strength, Ability::try_from_short("str")?);
        assert_eq!(Ability::Dexterity, Ability::try_from_short("dex")?);
        assert_eq!(Ability::Constitution, Ability::try_from_short("con")?);
        assert_eq!(Ability::Intelligence, Ability::try_from_short("int")?);
        assert_eq!(Ability::Wisdom, Ability::try_from_short("wis")?);
        assert_eq!(Ability::Charisma, Ability::try_from_short("cha")?);

        Ok(())
    }

    #[test]
    fn parse_valid_long_abiltities() -> Result<(), ParseError> {
        assert_eq!(Ability::Strength, Ability::try_from_string("strength")?);
        assert_eq!(Ability::Dexterity, Ability::try_from_string("dexterity")?);
        assert_eq!(
            Ability::Constitution,
            Ability::try_from_string("constitution")?
        );
        assert_eq!(
            Ability::Intelligence,
            Ability::try_from_string("intelligence")?
        );
        assert_eq!(Ability::Wisdom, Ability::try_from_string("wisdom")?);
        assert_eq!(Ability::Charisma, Ability::try_from_string("charisma")?);

        Ok(())
    }

    #[test]
    fn parse_hold_up_identity() -> Result<(), ParseError> {
        assert_eq!(
            Ability::try_from_string(
                Ability::try_from_short(Ability::try_from_string("strength")?.to_short().as_str())?
                    .to_string()
                    .as_str()
            )?,
            Ability::Strength
        );

        Ok(())
    }
}

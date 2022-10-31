use serde::{Deserialize, Serialize};

use crate::error::ParseError;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum FeatType {
    Class,
    Archetype,
    Ancestry,
    Skill,
    Bonus,
    General,
    ClassFeature,
}

impl From<FeatType> for String {
    fn from(value: FeatType) -> Self {
        match value {
            FeatType::Class => "class".into(),
            FeatType::Archetype => "archetype".into(),
            FeatType::Ancestry => "ancestry".into(),
            FeatType::Skill => "skill".into(),
            FeatType::Bonus => "bonus".into(),
            FeatType::General => "general".into(),
            FeatType::ClassFeature => "classfeature".into(),
        }
    }
}

impl From<FeatType> for usize {
    fn from(value: FeatType) -> Self {
        use FeatType::*;
        match value {
            Class => 0,
            Archetype => 1,
            Ancestry => 2,
            Skill => 3,
            Bonus => 4,
            General => 5,
            ClassFeature => 6,
        }
    }
}

impl TryFrom<usize> for FeatType {
    type Error = ParseError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use FeatType::*;

        match value {
            0 => Ok(Class),
            1 => Ok(Archetype),
            2 => Ok(Ancestry),
            3 => Ok(Skill),
            4 => Ok(Bonus),
            5 => Ok(General),
            6 => Ok(ClassFeature),
            _ => Err(ParseError::UnexpectedField {
                structure: "FeatType".to_string(),
                field: value.to_string(),
            }),
        }
    }
}

impl TryFrom<&str> for FeatType {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lower = value.to_lowercase();
        match lower.as_str() {
            "class" => Ok(Self::Class),
            "archetype" => Ok(Self::Archetype),
            "ancestry" => Ok(Self::Ancestry),
            "skill" => Ok(Self::Skill),
            "bonus" => Ok(Self::Bonus),
            "general" => Ok(Self::General),
            "classfeature" => Ok(Self::ClassFeature),
            _ => Err(ParseError::UnexpectedField {
                structure: "FeatType".to_string(),
                field: lower,
            }),
        }
    }
}

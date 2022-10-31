use serde::{Deserialize, Serialize};

use crate::error::ParseError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum Skill {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore,
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
}

impl Skill {
    pub fn try_from_string(s: &str) -> Result<Self, ParseError> {
        use Skill::*;
        let lower = s.to_lowercase();
        match lower.as_str() {
            "acrobatics" => Ok(Acrobatics),
            "arcana" => Ok(Arcana),
            "athletics" => Ok(Athletics),
            "crafting" => Ok(Crafting),
            "deception" => Ok(Deception),
            "diplomacy" => Ok(Diplomacy),
            "intimidation" => Ok(Intimidation),
            "lore" => Ok(Lore),
            "medicine" => Ok(Medicine),
            "nature" => Ok(Nature),
            "occultism" => Ok(Occultism),
            "performance" => Ok(Performance),
            "religion" => Ok(Religion),
            "society" => Ok(Society),
            "stealth" => Ok(Stealth),
            "survival" => Ok(Survival),
            _ => Err(ParseError::UnexpectedField {
                structure: "Skill".into(),
                field: lower,
            }),
        }
    }

    pub fn to_string(&self) -> String {
        use Skill::*;
        String::from(match self {
            Acrobatics => "acrobatics",
            Arcana => "arcana",
            Athletics => "athletics",
            Crafting => "crafting",
            Deception => "deception",
            Diplomacy => "diplomacy",
            Intimidation => "intimidation",
            Lore => "lore",
            Medicine => "medicine",
            Nature => "nature",
            Occultism => "occultism",
            Performance => "performance",
            Religion => "religion",
            Society => "society",
            Stealth => "stealth",
            Survival => "survival",
        })
    }

    pub fn try_from_short(s: &str) -> Result<Self, ParseError> {
        let lower = s.to_lowercase();
        use Skill::*;
        match lower.as_str() {
            "acr" => Ok(Acrobatics),
            "arc" => Ok(Arcana),
            "ath" => Ok(Athletics),
            "cra" => Ok(Crafting),
            "dec" => Ok(Deception),
            "dip" => Ok(Diplomacy),
            "int" => Ok(Intimidation),
            "lor" => Ok(Lore),
            "med" => Ok(Medicine),
            "nat" => Ok(Nature),
            "occ" => Ok(Occultism),
            "prf" => Ok(Performance),
            "rel" => Ok(Religion),
            "soc" => Ok(Society),
            "ste" => Ok(Stealth),
            "sur" => Ok(Survival),
            _ => Err(ParseError::UnexpectedField {
                structure: "Skill".into(),
                field: lower,
            }),
        }
    }

    pub fn to_short(&self) -> String {
        use Skill::*;
        String::from(match self {
            Acrobatics => "acr",
            Arcana => "arc",
            Athletics => "ath",
            Crafting => "cra",
            Deception => "dec",
            Diplomacy => "dip",
            Intimidation => "int",
            Lore => "lor",
            Medicine => "med",
            Nature => "nat",
            Occultism => "occ",
            Performance => "prf",
            Religion => "rel",
            Society => "soc",
            Stealth => "ste",
            Survival => "sur",
        })
    }
}

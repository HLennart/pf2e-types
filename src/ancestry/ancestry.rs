use crate::{
    pf_trait::Rarity,
    statistic::{AbilityBoost, AbilityFlaw, Size, Vision},
};

#[derive(Clone, Debug)]
pub struct Ancestry {
    pub id: usize,
    pub name: String,
    pub img_path: String,
    pub boosts: Vec<AbilityBoost>,
    pub description: String,
    pub flaws: Vec<AbilityFlaw>,
    pub hp: usize,
    pub reach: usize,
    pub size: Size,
    pub source: String,
    pub speed: usize,
    pub rarity: Rarity,
    pub vision: Vision,
    pub traits: Vec<String>,
    pub languages: Vec<String>,
    pub additional_languages: Vec<String>,
    pub language_count: usize,
}

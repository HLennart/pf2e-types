use serde::{Deserialize, Serialize};

use crate::{
    pf_trait::Rarity,
    proficiency::{DefenseProficiency, Proficiency, SavingThrows, WeaponProficiency},
    statistic::{Ability, Skill},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub id: usize,
    pub name: String,
    pub image_path: String,
    pub class_dc: usize,
    pub description: String,
    pub hp: usize,
    pub key_ability: Vec<Ability>,
    pub perception: Proficiency,
    pub class_feat_levels: Vec<usize>,
    pub skill_feat_levels: Vec<usize>,
    pub general_feat_levels: Vec<usize>,
    pub source: String,
    pub additional_skills: usize,
    pub trained_skills: Vec<Skill>,
    pub rarity: Rarity,
    pub saving_throws: SavingThrows,
    pub defense_proficiencies: DefenseProficiency,
    pub weapon_proficiencies: WeaponProficiency,
}

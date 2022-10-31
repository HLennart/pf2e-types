use serde::{Deserialize, Serialize};

use crate::{
    action::{ActionCategory, ActionType},
    pf_trait::Rarity,
};

use super::feat_type::FeatType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feat {
    pub id: usize,
    pub name: String,
    pub image_path: String,
    pub action_category: ActionCategory,
    pub action_type: ActionType,
    pub actions: Option<usize>,
    pub description: String,
    pub feat_type: FeatType,
    pub level: Option<usize>,
    pub prerequisites: Vec<String>,
    pub rarity: Rarity,
    pub traits: Vec<String>,
    pub foundry_id: String,
}

impl PartialEq for Feat {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

use crate::{action::Actions, pf_trait::Rarity};

#[derive(Debug, Clone)]
pub struct ClassFeature {
    pub id: usize,
    pub image_path: String,
    pub name: String,
    pub classes: Vec<String>,
    pub level: usize,
    pub actions: Actions,
    pub prerequisites: Vec<String>,
    pub source: Option<String>,
    pub rarity: Rarity,
    pub traits: Vec<String>,
}

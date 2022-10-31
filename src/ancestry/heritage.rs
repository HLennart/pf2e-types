use crate::pf_trait::Rarity;

#[derive(Clone, Debug)]
pub struct Heritage {
    pub id: usize,
    pub name: String,
    pub img_path: String,
    pub ancestry: Option<String>,
    pub description: String,
    pub rarity: Rarity,
    pub traits: Vec<String>,
    pub source: String,
}

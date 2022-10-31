use super::Proficiency;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct DefenseProficiency {
    pub heavy_armor: Proficiency,
    pub medium_armor: Proficiency,
    pub light_armor: Proficiency,
    pub unarmored: Proficiency,
}

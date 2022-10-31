use serde::{Deserialize, Serialize};

use super::Proficiency;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct DefenseProficiency {
    pub heavy_armor: Proficiency,
    pub medium_armor: Proficiency,
    pub light_armor: Proficiency,
    pub unarmored: Proficiency,
}

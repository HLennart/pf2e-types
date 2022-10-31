use serde::{Deserialize, Serialize};

use super::Proficiency;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct WeaponProficiency {
    pub advanced: Proficiency,
    pub martial: Proficiency,
    pub simple: Proficiency,
    pub unarmed: Proficiency,
}

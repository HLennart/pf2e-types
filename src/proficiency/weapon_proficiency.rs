use super::Proficiency;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct WeaponProficiency {
    pub advanced: Proficiency,
    pub martial: Proficiency,
    pub simple: Proficiency,
    pub unarmed: Proficiency,
}

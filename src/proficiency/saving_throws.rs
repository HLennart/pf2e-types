use super::Proficiency;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub struct SavingThrows {
    pub fortitude: Proficiency,
    pub reflex: Proficiency,
    pub will: Proficiency,
}

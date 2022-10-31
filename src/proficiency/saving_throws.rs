use serde::{Deserialize, Serialize};

use super::Proficiency;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct SavingThrows {
    pub fortitude: Proficiency,
    pub reflex: Proficiency,
    pub will: Proficiency,
}

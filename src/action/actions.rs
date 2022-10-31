use crate::error::ParseError;

#[derive(Copy, Clone, Debug)]
pub enum Actions {
    None,
    One,
    Two,
    Three,
}

// impl Actions {
//     pub fn to_string(&self) -> String {
//         match self {
//             Actions::None => "".into(),
//             Actions::One => "1 Action".into(),
//             Action::Two => ""
//         }
//     }
// }

impl TryFrom<Option<usize>> for Actions {
    type Error = ParseError;

    fn try_from(value: Option<usize>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Actions::None),
            Some(actions) => match actions {
                1 => Ok(Actions::One),
                2 => Ok(Actions::Two),
                3 => Ok(Actions::Three),
                _ => Err(ParseError::UnexpectedField {
                    structure: "Actions".to_string(),
                    field: format!("{:?}", value),
                }),
            },
        }
    }
}

impl From<Actions> for Option<usize> {
    fn from(value: Actions) -> Self {
        match value {
            Actions::None => None,
            Actions::One => Some(1),
            Actions::Two => Some(2),
            Actions::Three => Some(3),
        }
    }
}

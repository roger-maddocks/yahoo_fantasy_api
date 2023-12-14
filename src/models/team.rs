use serde::{Deserialize, Serialize};
use crate::models::player::{NhlFranchise, Player, Position};
use crate::models::player::NhlFranchise::ColoradoAvalanche;

#[derive(Debug, Serialize, Deserialize, Hash, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub abbreviation: String,
    #[serde(rename = "id")]
    pub msf_id: u16//NhlFranchise
}
impl Team {
    pub fn new(
        abbreviation: String,
        msf_id: u16
    ) -> Team {
        Self {
            abbreviation,
            msf_id
        }
    }
    pub fn default() -> Team {
        Self {
            abbreviation: "".to_string(),
            msf_id: 0,
        }
    }
}


impl Clone for Team {
    fn clone(&self) -> Self {
        Team {
            abbreviation: self.abbreviation.clone(),
            msf_id: self.msf_id.clone()
            // franchise: Default::default(),
        }
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Team) -> bool {
        self.abbreviation == other.abbreviation
    }
}

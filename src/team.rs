use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub abbreviation: String,
    pub full_name: Option<String>
}
impl Clone for Team {
    fn clone(&self) -> Self {
        Team
        {
            abbreviation: self.abbreviation.clone(),
            full_name: self.full_name.clone()
        }
    }
}


impl PartialEq for Team {
    fn eq(&self, other:&Team) -> bool {
        self.abbreviation == other.abbreviation
    }
}

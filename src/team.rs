use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
enum Division{
    Atlantic,
    Metropolitan,
    Central,
    Pacific
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub abbreviation: String,
    pub full_name: Option<String>,
    pub city: String,
    pub mascot: String,
    pub home_arena: String,
    pub division: Division
}
impl Clone for Team {
    fn clone(&self) -> Self {
        Team
        {
            abbreviation: self.abbreviation.to_string(),
            full_name: Some(self.full_name.to_string()),
            city: self.city.to_string(),
            mascot: self.mascot.to_string(),
            home_arena: self.mascot.to_string(),
            division: self.division.clone(),
        }
    }
}


impl PartialEq for Team {
    fn eq(&self, other:&Team) -> bool {
        self.abbreviation == other.abbreviation
    }
}

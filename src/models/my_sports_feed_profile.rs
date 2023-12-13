#[derive(Debug, PartialEq)]
pub struct MySportsFeedProfile {
    api_key: String,
    api_pw: String,
}

impl MySportsFeedProfile {
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::default()
    }
}

#[derive(Default)]
pub struct ProfileBuilder {
    api_key: String,
    api_pw: String,
}

impl ProfileBuilder {
    pub fn new() -> ProfileBuilder {
        ProfileBuilder {
            api_key: String::from("163db9fd-bdbe-4bd4-a4b8-15324f"),
            api_pw: String::from("MYSPORTSFEEDS"),
        }
    }
    pub fn name(mut self, api_key: String, api_pw: String) -> ProfileBuilder {
        self.api_key = api_key;
        self.api_pw = api_pw;
        self
    }
    pub fn build(self) -> MySportsFeedProfile {
        MySportsFeedProfile {
            api_key: self.api_key,
            api_pw: self.api_pw,
        }
    }
}
// impl Default for MySportsFeedProfile {
// fn default() -> Self {
//     Self {
//         api_key: "163db9fd-bdbe-4bd4-a4b8-15324f".to_string(),
//         api_pw: "MYSPORTSFEEDS".to_string()
//     }
// }
// }

// pub fn GetApiKey (&self) -> String { self.api_key }
// pub fn GetApiPw (&self) -> String { self.api_pw }


pub struct Arena {
    pub name: String,
    pub theme: String
}

impl Arena {
    pub fn new(name: &str, theme: String) -> Self {
        Self {
            name: name.to_string(),
            theme
        }
    }
}
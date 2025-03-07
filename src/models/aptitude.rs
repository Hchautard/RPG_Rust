
pub struct Aptitude {
    pub name: String,
    pub description: String,
    pub pp: i32,
    pub power: f32
}

impl Aptitude {
    pub fn new(name: &str, description: String, pp:i32, power: f32 ) -> Self {
        Self {
            name: name.to_string(),
            description,
            pp,
            power
        }
    }
}
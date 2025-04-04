use super::combo::Combo;


pub struct Fight {
    pub number_of_round: i32,
    pub theme: String,
    pub scoring: i32,
}

impl Fight {
    pub fn new(number_of_round: i32, theme: String, scoring:i32) -> Self {
        Self {
            number_of_round,
            theme,
            scoring
        }
    }

    pub fn verifyThemeCoherence(&self, combo: &Combo) -> bool {
        return self.theme == combo.theme;
    }

    
}
pub struct Player {
    pub caracter: Caracter,
    pub badge: Badge,
    pub inventory: Vec<String>, // Type Ingredient
    pub level: u32,
    pub reputation: u32,
    pub specialization: Vec<String>, // Type Specialization
    pub aptitudes: Vec<String>, // Type Aptitude
}

impl Player {
    pub fn new(name: &str, style: &str, badge: Badge, inventory: Vec<String>, specialization: Vec<String>, aptitudes: Vec<String>) -> Self {
        Self {
            caracter: Caracter::new(name, style, 100, 50, 0),
            level: 1,
            reputation: 0,
            inventory: inventory,
            badge: badge,
            specialization: specialization,
            aptitudes: aptitudes,
        }
    }

    pub fn gain_xp(&mut self, xp: u32) {
        self.reputation += xp;
        println!("{} gagne {} XP!", self.caracter.name, xp);

        if self.reputation >= 100 {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;
        self.caracter.hp += 10;
        println!("🎉 {} passe au niveau {} !", self.caracter.name, self.level);
    }
}

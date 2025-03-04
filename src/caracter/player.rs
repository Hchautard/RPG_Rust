pub struct Player {
    pub caracter: Caracter,
    pub level: u32,
    pub reputation: u32,
}

impl Player {
    pub fn new(name: &str, style: &str) -> Self {
        Self {
            caracter: Caracter::new(name, style, 100, 50, 0),
            level: 1,
            reputation: 0,
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

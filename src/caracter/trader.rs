pub struct Trader {
    pub pnj: Pnj,
    pub ingredients : Vec<String>,// TODO a mettre le type ingredient
}

impl Trader {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, job: &str, dialogs: Vec<String>, ingredients: Vec<String>) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            ingredients: ingredients
        }
    }

    pub fn buy() {
        // TODO
    }

    pub fn sell() {
        // TODO
    }
}

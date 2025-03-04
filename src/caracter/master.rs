pub struct Master {
    pub pnj: Pnj,
    pub badge : String, // Badge Type
    pub attacks: Vec<String>,
}

impl Trader {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, job: &str, dialogs: Vec<String>, badge: String, attacks: Vec<String>) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            badge,
            attacks
        }
    }

    pub fn play(){
        // TODO
    }
}

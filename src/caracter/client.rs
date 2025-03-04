pub struct Client {
    pub pnj: Pnj,
    pub tips: i32,
    pub advices: Vec<String>,
}

impl Client {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, job: &str, dialogs: Vec<String>, tips: i32, advices: Vec<String>) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            tips: tips,
            advices: advices
        }
    }

    pub fn giveReputation() {
        // TODO
    }
}

use super::pnj::Pnj;

pub struct Bouncer {
    pub pnj: Pnj,
    pub enigmas : Vec<String>,
}

impl Bouncer {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, job: &str, dialogs: Vec<String>, enigmas: Vec<String>) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            enigmas: Vec::new(),
        }
    }

    pub fn giveEnigma(){

    }

    pub fn verifyEnigma(){

    }

    pub fn bounce(){
        
    }
}

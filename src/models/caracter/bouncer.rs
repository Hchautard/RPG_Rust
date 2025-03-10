use std::string;

use super::pnj::Pnj;

pub struct Bouncer {
    pub pnj: Pnj,
    pub enigmas : Vec<String>,
    pub answer : String,
}

impl Bouncer {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, job: &str, dialogs: Vec<String>, enigmas: Vec<String>, answer: String) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            enigmas: Vec::new(),
            answer
        }
    }

    pub fn giveEnigma(){

    }
    
    pub fn verifyEnigma(&self, solution: &str){
        if(solution == self.answer){
            println!("Good answer");
        } else {
            self.bounce();
        }
    }

    pub fn bounce(&self){
        
    }
}

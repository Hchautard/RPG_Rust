
mod services{
    pub mod json_loader;
    pub mod displayer;
}
mod models{
    pub mod badge;
    pub mod ingredient;
    pub mod combo;
    pub mod aptitude;
    pub mod fight;
    pub mod arena;
    pub mod caracter{
        pub mod caracter;
        pub mod bouncer;
        pub mod pnj;
        pub mod player;
        pub mod client;
        pub mod trader;
        pub mod master;
    }
}

use std::io::{self};
use crate::services::displayer::Displayer;

fn main() -> io::Result<()> {
    let mut displayer = Displayer::new()?;
    displayer.show_menu()?;
    displayer.cleanup()?;
    Ok(())
}

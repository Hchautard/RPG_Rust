pub struct Round {
    pub name: String,
    pub theme: String,
    pub badge: String,
    pub master: String,
    pub clients: Vec<String>,
    pub bouncer: String,
    pub trader: String,
}

impl Round {
    pub fn new(name: &str, theme: &str, badge: &str, master: &str, bouncer: &str, trader: &str, clients: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            theme: theme.to_string(),
            badge: badge.to_string(),
            master: master.to_string(),
            bouncer: bouncer.to_string(),
            trader: trader.to_string(),
            clients,
        }
    }
}

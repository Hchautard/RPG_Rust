use crate::models::aptitude::Aptitude;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

/// Interface utilisateur en mode terminal
pub(crate) struct Displayer {
    // Terminal configuré avec crossterm
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Displayer {
    // On crée un nouveau terminal en mode plein écran
    pub fn new() -> io::Result<Self> {
        // Configuration du terminal en mode raw
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        Ok(Self { terminal })
    }

    // Affiche le menu principal et gère la navigation
    pub fn show_menu(&mut self, aptitudes: &[Aptitude]) -> io::Result<()> {
        loop {
            // Rendu du menu principal

            self.terminal.draw(|f| {
                let size = f.size();
                // Bloc avec titre et bordures

                let block = Block::default()
                    .title("🔥 RPG Rust - Menu Principal 🔥")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double);
                // Options du menu

                let menu_items = vec!["1. Nouvelle Partie", "2. Charger Partie", "3. Quitter"];

                let paragraph = Paragraph::new(menu_items.join("\n"))
                    .alignment(Alignment::Center)
                    .block(block);

                f.render_widget(paragraph, size);
            })?;

            // Gestion des événements clavier

            if event::poll(std::time::Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('1') => {
                            println!("Demarrage d'une nouvelle partie...");
                            break;
                        }
                        KeyCode::Char('2') => {
                            println!("Chargement de la sauvegarde...");
                            break;
                        }
                        KeyCode::Char('3') | KeyCode::Esc => {
                            println!("Fermeture du jeu...");
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
    // Affiche la liste des aptitudes disponibles

    pub fn display_aptitudes(&mut self, aptitudes: &[Aptitude]) -> io::Result<()> {
        loop {
            // Rendu de la liste des aptitudes

            self.terminal.draw(|f| {
                let size = f.size();
                // Bloc avec titre stylisé

                let block = Block::default()
                    .title("🍸 Aptitudes des Bartenders 🍸")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                // Formatage des aptitudes

                let aptitude_list: Vec<String> = aptitudes
                    .iter()
                    .map(|apt| {
                        format!(
                            "🔹 {} - {} (PP: {}, Power: {})",
                            apt.name, apt.description, apt.pp, apt.power
                        )
                    })
                    .collect();

                let paragraph = Paragraph::new(aptitude_list.join("\n\n"))
                    .alignment(Alignment::Left)
                    .block(block);

                f.render_widget(paragraph, size);
            })?;
            // pour pouvoir sortir avec echa ou 'q'
            if event::poll(std::time::Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    // Nettoie et restaure le terminal
    pub fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}

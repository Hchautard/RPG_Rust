use std::io::{self, stdout};
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use crate::models::aptitude::Aptitude;

pub(crate) struct Displayer {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Displayer {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        Ok(Self { terminal })
    }

    pub fn show_menu(&mut self, aptitudes: &[Aptitude]) -> io::Result<()> {
        loop {
            self.terminal.draw(|f| {
                let size = f.size();

                let block = Block::default()
                    .title("🔥 RPG Rust - Menu Principal 🔥")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double);

                let menu_items = vec![
                    "1. Nouvelle Partie",
                    "2. Charger Partie",
                    "3. Quitter",
                ];

                let paragraph = Paragraph::new(menu_items.join("\n"))
                    .alignment(Alignment::Center)
                    .block(block);

                f.render_widget(paragraph, size);
            })?;

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

    pub fn display_aptitudes(&mut self, aptitudes: &[Aptitude]) -> io::Result<()> {
        loop {
            self.terminal.draw(|f| {
                let size = f.size();

                let block = Block::default()
                    .title("🍸 Aptitudes des Bartenders 🍸")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);

                let aptitude_list: Vec<String> = aptitudes
                    .iter()
                    .map(|apt| format!("🔹 {} - {} (PP: {}, Power: {})", apt.name, apt.description, apt.pp, apt.power))
                    .collect();

                let paragraph = Paragraph::new(aptitude_list.join("\n\n"))
                    .alignment(Alignment::Left)
                    .block(block);

                f.render_widget(paragraph, size);
            })?;

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

    pub fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}

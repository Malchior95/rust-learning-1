use crate::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use datetime::LocalDateTime;
use futures::{FutureExt, StreamExt};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

use super::{Page, not_found::NotFoundPage};

#[derive(Debug)]
pub struct LandingPage {}
impl LandingPage {
    pub async fn handle_page(&self, app: &mut App, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| self.draw(app, frame))?;
        self.handle_crossterm_events(app).await?;
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&self, app: &mut App, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
            (_, KeyCode::Char('n')) => app.page = Some(Page::NotFound(NotFoundPage::default())),
            // Add other key handlers here.
            _ => {}
        }
    }

    fn draw(&self, app: &mut App, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    async fn handle_crossterm_events(&self, app: &mut App) -> Result<()> {
        tokio::select! {
            event = app.event_stream.next().fuse() => {
                match event {
                    Some(Ok(evt)) => {
                        match evt {
                            Event::Key(key)
                                if key.kind == KeyEventKind::Press
                                    =>self.on_key_event(app, key),
                            Event::Mouse(_) => {}
                            Event::Resize(_, _) => {}
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                // Sleep for a short duration to avoid busy waiting.
            }
        }
        Ok(())
    }
}

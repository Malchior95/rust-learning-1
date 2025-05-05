use crate::{App, ui::landing::LandingPage};
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

use super::{Page, landing};

#[derive(Debug)]
pub struct NotFoundPage {
    created_at: LocalDateTime,
}
impl Default for NotFoundPage {
    fn default() -> Self {
        Self {
            created_at: LocalDateTime::now(),
        }
    }
}
impl NotFoundPage {
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
            // Add other key handlers here.
            _ => {}
        }
    }

    fn draw(&self, app: &mut App, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = format!(
            "Not found!!!\n{:?}\n{:?}",
            self.created_at,
            LocalDateTime::now()
        );
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    async fn handle_crossterm_events(&self, app: &mut App) -> Result<()> {
        if self.created_at.add_seconds(5) < LocalDateTime::now() {
            app.page = Some(Page::LandingPage(LandingPage {}));
            return Ok(());
        }

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

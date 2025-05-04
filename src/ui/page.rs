use crate::App;
use color_eyre::Result;
use ratatui::Frame;

pub trait Page {
    fn draw(&self, app: &mut App, frame: &mut Frame);

    async fn handle_crossterm_events(&self, app: &mut App) -> Result<()>;
}

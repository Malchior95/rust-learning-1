use color_eyre::Result;
use crossterm::event::EventStream;
use ratatui::DefaultTerminal;
use ui::page::Page;

mod router;
mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    // Event stream.
    event_stream: EventStream,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        while self.running {
            let current_route = router::routes::Routes::LandingPage;

            let page = router::route(current_route);

            terminal.draw(|frame| page.draw(&mut self, frame))?;
            page.handle_crossterm_events(&mut self).await?;
        }
        Ok(())
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

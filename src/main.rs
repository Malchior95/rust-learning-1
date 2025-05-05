use color_eyre::Result;
use crossterm::event::EventStream;
use ratatui::DefaultTerminal;
use ui::Page;

mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    // Event stream.
    event_stream: EventStream,
    page: Option<Page>, //this has to be Option to allow me to 'move' value outside of App
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        App {
            running: true,
            event_stream: EventStream::default(),
            page: Some(Page::LandingPage(ui::landing::LandingPage {})),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            ui::handle_page(&mut self, &mut terminal).await?;
        }
        Ok(())
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

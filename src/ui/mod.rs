use crate::App;
use color_eyre::{Result, eyre::Report, eyre::eyre};
use landing::LandingPage;
use not_found::NotFoundPage;
use ratatui::{DefaultTerminal, Frame};

pub mod landing;
pub mod not_found;

#[derive(Debug)]
pub enum Page {
    LandingPage(LandingPage),
    About,
    News,
    NotFound(NotFoundPage),
}
impl Default for Page {
    fn default() -> Self {
        Self::LandingPage(LandingPage {})
    }
}

pub async fn handle_page(app: &mut App, terminal: &mut DefaultTerminal) -> Result<()> {
    let page = app.page.take(); //this is some rust magic, but it allows me to move 'page' out of
    //'app'

    match page.as_ref().unwrap() {
        Page::LandingPage(landing_page) => landing_page.handle_page(app, terminal).await?,
        Page::NotFound(not_found_page) => not_found_page.handle_page(app, terminal).await?,
        _ => todo!()
    }
    if let None = app.page {
        app.page = page;
    }
    Ok(())
}

//pub struct Page {
//    pub draw: fn(app: &mut App, frame: &mut Frame),
//    pub handle_crossterm_events: fn(app: &mut App) -> Result<()>,
//}

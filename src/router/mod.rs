use routes::Routes;

use crate::ui::{landing::LandingPage, not_found::NotFoundPage, page::Page};
pub mod routes;

pub fn route(route: Routes) -> Box<dyn Page> {
    match route {
        Routes::LandingPage => Box::new(LandingPage {}),
        _ => Box::new(NotFoundPage {}),
    }
}

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};

mod handlers;
mod models;
/// A simple Spin HTTP component.
#[http_component]
fn handle_url_shortener(req: Request) -> Result<Response> {
    
    let mut router = Router::default();
    router.get("/all", handlers::get_all_links);
    router.post("/create", handlers::create_short_url);
    router.get("/:term", handlers::open_link);
    router.any("*", handlers::not_found);

    router.handle(req)
}



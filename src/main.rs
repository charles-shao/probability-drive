use draw::result::Weighted;
use rocket::fs::{FileServer, NamedFile};
use std::path::Path;

mod draw;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/playground")]
async fn playground() -> Option<NamedFile> {
    NamedFile::open(Path::new("pages/playground").join("index.html"))
        .await
        .ok()
}

#[get("/playground/scripts.js")]
async fn playground_scripts() -> Option<NamedFile> {
    NamedFile::open(Path::new("pages/playground").join("scripts.js"))
        .await
        .ok()
}

#[get("/playground/styles.css")]
async fn playground_styles() -> Option<NamedFile> {
    NamedFile::open(Path::new("pages/playground").join("styles.css"))
        .await
        .ok()
}

#[get("/results.json")]
async fn results() -> String {
    let results: Weighted = Weighted::new(10);

    results.to_json()
}

#[get("/echo")]
fn echo_stream(ws: ws::WebSocket) -> ws::Stream!['static] {
    let ws = ws.config(ws::Config {
        max_write_buffer_size: 5,
        ..Default::default()
    });

    ws::Stream! { ws =>
        for await message in ws {
            yield message?;
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                results,
                playground,
                playground_scripts,
                playground_styles,
                echo_stream
            ],
        )
        .mount("/public", FileServer::from("static/"))
}

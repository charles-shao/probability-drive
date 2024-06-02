use drivers::weighted::Weighted;
use probability_drive::ThreadPool;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use serde_json;

mod drivers;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool: ThreadPool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let roll: &[u8; 25] = b"GET /roll.json HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", root_response())
    } else if buffer.starts_with(roll) {
        let weighted_drive: Weighted = Weighted::new(10);

        ("HTTP/1.1 200 OK", weighted_drive.to_json())
    } else {
        ("HTTP/1.1 404 NOT FOUND", not_found_response())
    };

    let length: usize = contents.len();
    let response: String = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn not_found_response() -> String {
    let json: serde_json::Value = serde_json::json!({
        "status": "not found"
    });

    json.to_string()
}

fn root_response() -> String {
    drivers::weighted::chance_map_to_json()
}

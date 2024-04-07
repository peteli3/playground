use log::info;
use my_server::ThreadPool;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    info!("Starting server");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let success_html = r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>Hello!</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <p>Hi from Rust</p>
        </body>
    </html>
    "#;

    let not_found_html = r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>Hello!</title>
        </head>
        <body>
            <h1>Oops!</h1>
            <p>Sorry, I don't know what you're asking for.</p>
        </body>
    </html>
    "#;

    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", success_html),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", success_html)
        }
        _ => ("HTTP/1.1 404 NOT FOUND", not_found_html),
    };

    let length = contents.len();
    let length_line = format!("Content-Length: {length}");
    let response = format!("{status_line}\r\n{length_line}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    info!("{request_line} -> {status_line}");
}

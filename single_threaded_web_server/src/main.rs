use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    if let Err(err) = run_server() {
        eprintln!("Error: {}", err);
    }
}

fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server running on http://127.0.0.1:7878/");

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(line) => line?,
        None => return Ok(()), // Ignore empty requests
    };

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            return Ok(());
        }
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    println!("Connection established!");

    Ok(())
}

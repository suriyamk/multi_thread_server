use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration,
};

use multi_thread_server::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(_stream);
        });
    }

    println!("Shutting down!.")
}

fn handle_connection(mut stream: TcpStream) {
    let buffer_data = BufReader::new(&stream);
    // let http_request: Vec<_> = buffer_data
    //     .lines()
    //     .map(|x| x.unwrap())
    //     .take_while(|x| !x.is_empty())
    //     .collect();
    let http_request = buffer_data.lines().next().unwrap().unwrap();

    // let (status_line, file_name) = if http_request == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
    let (status_line, file_name) = match &http_request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents: String = fs::read_to_string(file_name).unwrap();
    let content_len = contents.len();

    let http_response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
    stream.write_all(http_response.as_bytes()).unwrap();
}


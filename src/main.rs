use std::{ fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main() {
   let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
       let buffer_data =  BufReader::new(&stream);
       let http_request:  Vec<_> = buffer_data.lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect();

    let status_line= "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let content_len = contents.len();

    let http_response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
    stream.write_all(http_response.as_bytes()).unwrap();
}

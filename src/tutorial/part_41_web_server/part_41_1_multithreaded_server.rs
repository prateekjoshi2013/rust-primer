use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    // active connection count
    let mut active_requests = Arc::new(Mutex::new(0));
    // let stream = listener.accept();
    // println!("the stream {:?} \n the socket {:?}",stream.as_ref().unwrap().1,stream.as_ref().unwrap().0);
    for stream in listener.incoming() {
        let active_requests = Arc::clone(&active_requests);
        let stream = stream.unwrap();
        thread::spawn(move || {
            {
                let mut connection = active_requests.lock().unwrap();
                *connection += 1;
                if *connection >= 3 {
                    thread::sleep(Duration::from_secs(2)); // needed to let other threads take over
                }
            } // block is needed to remove connection variable to release lock for other thread to use it

            handle_connection(stream);

            {
                let mut connection = active_requests.lock().unwrap();
                *connection -= 1;
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|lines| !lines.is_empty())
    //     .collect::<Vec<String>>();
    // println!("http_request: {:?}", http_request);

    let request_line = buf_reader.lines().next();
    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" | "GET /index HTTP/1.1" => {
            thread::sleep(Duration::from_secs(4)); // slower request
            (Some("HTTP/1.1 200 OK\r\n"), Some("index.html"))
        }
        "GET /index/1 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index1.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")),
    };

    /*
    Response Syntax

    Http version Status-Code Reason-Phrase CRLF
    headers CRLF
    message-body

    ex: HTTP/1.1 200 OK\r\n\r\n
    */
    let contents = fs::read_to_string(format!("src/tutorial/{}", file_name.unwrap())).unwrap();
    let length = contents.len();
    let response = format!(
        "{} Contents-Length: {}\r\n\r\n{}",
        status_line.unwrap(),
        length,
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

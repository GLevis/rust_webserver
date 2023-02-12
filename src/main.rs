use rust_server::{ThreadPool, PoolCreationError};

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(var) => var,
        Err(_) => panic!("Failed to bind"),
    };
    let pool = match ThreadPool::build(4) {
        Ok(thread_pool) => { 
            thread_pool
        }
        Err(error) => match error {
            PoolCreationError::Negative => panic!("Thread Pool is negative!"),
            PoolCreationError::Zero => panic!("Thread Pool is zero!"),
        }
    };

    for stream in listener.incoming() {
        pool.execute(|| {
            match stream {
                Ok(stream) => {
                    handle_connection(stream);
                }
                Err(_) => { panic!("Connection failed!"); }
            };
        });
    }
}

fn handle_get(mut stream: TcpStream, file: &str) {
    let mut chars = file.chars();
    chars.next();
    let contents = match fs::read_to_string(chars.as_str()) {
        Ok(contents) => contents,
        Err(_) => match fs::read_to_string("404.html") {
                    Ok(string) => string,
                    Err(_) => { panic!("Failed to read 404.html");}
                  }
    };
    let length = contents.len();
    let status_line = "HTTP/1.1 200 OK";
    let response =
       format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Unable to write to stream");
    stream.flush().expect("Failed to flush stream");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
     
    if let request = &request_line[..].split(" ").collect::<Vec<&str>>() {
        match request[0] {
            "GET" => handle_get(stream, &request[1]),
            _ => println!("{}", request[0])
        };
    };

    //let contents = fs::read_to_string(filename).unwrap();
    //let length = contents.len();

    //let response =
     //   format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //stream.write_all(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
}

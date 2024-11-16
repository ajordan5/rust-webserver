use core::panic;
use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration  
};
use webserver::ThreadPool;  

fn main() {
    // bind tcp listener to localhost at port 7878 and spin up a threadpool to 
    // handle connection requests
    //
    // TODO: lots of unwraps happening here, improve this to gracefully handle errors
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool_result = ThreadPool::build(4);
    let pool = match pool_result {
        Ok(pool) => pool,
        Err(thread_err) => panic!("Problem setting up thread pool: {thread_err:?}")
    };
    for stream_res in listener.incoming() {
        match stream_res {
            Ok(stream) => {

                pool.execute(|| {
                    handle_connections(stream);
                });
            },
            Err(msg) => println!("Issue with steam {msg:?}")
        }
    }
}

fn handle_connections(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // let _http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    println!("Request: {request_line:#?}");
    let (status, file) = match &request_line[..] {

        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK",  "test.html")}
        "GET / HTTP/1.1" => {
            ("HTTP/1.1 200 OK",  "test.html")}
        _ => {
            ("HTTP/1.1 404 NOT FOUND",  "oops.html")
        }  
    };


    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

}


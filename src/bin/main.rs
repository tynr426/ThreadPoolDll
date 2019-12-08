 use std::thread;
 use std::io::prelude::*;
 use std::net::TcpListener;
 use std::net::TcpStream;
use TheadPoolDll::ThreadPool;
fn main() {
     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection( stream: TcpStream) {}

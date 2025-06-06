/* based on: https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html; */
use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read};
fn main() -> ()
{
    let listener = TcpListener::bind("127.0.0.1:4545").unwrap();
    println!("server_info: The rust socket listener in {:?}", listener);
    for stream in listener.incoming() 
    {
        foo(stream);
    }    
}
fn foo(stream: Result<TcpStream, Error>) -> () 
{   
    let mut stream_buffer: [u8;512] = [0;512]; //512 bytes max!
    stream.ok()
    .unwrap().read(&mut stream_buffer)
    .expect("Error wen read buffer!");
    println!("{}", str::from_utf8(&stream_buffer).ok().unwrap().replace("\\0",""));
}


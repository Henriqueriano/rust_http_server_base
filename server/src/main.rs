/* based on: https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html; */
use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read};

fn main() -> ()
{
    let listener: TcpListener = TcpListener::bind("0.0.0.0:4343").unwrap();
    clearscreen::clear().expect("Failed on clear!");
    println!("server_info: The rust socket listener in {:?}", listener);
    for stream in listener.incoming() 
    {
        print_when_listen(stream);
    }    
}
fn print_when_listen(stream: Result<TcpStream, Error>) -> () 
{   
    let mut stream_buffer: [u8;512] = [0;512]; //512 bytes max!
    stream.ok()
    .unwrap().read(&mut stream_buffer)
    .expect("Error wen read buffer!");
    print!("{}", str::from_utf8(&stream_buffer).ok().unwrap().trim().replace("\\0",""));
}


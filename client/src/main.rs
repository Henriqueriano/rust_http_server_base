use std::net::{TcpStream, Shutdown};
use std::io::{Write};
use std::io;
fn main() -> io::Result<()>
{   
    //https://doc.rust-lang.org/std/io/#standard-input-and-output
    loop
    {
        let mut conn: TcpStream = TcpStream::connect("127.0.0.1:4343").expect("Failed!");
        let mut msg: String = String::new();
        println!("Type what do u want to send: ");
        io::stdin().read_line(&mut msg).unwrap();
        let _ = conn.write(msg.as_bytes());
        conn.shutdown(Shutdown::Both).expect("Failed when shutdown!")
    }   
}

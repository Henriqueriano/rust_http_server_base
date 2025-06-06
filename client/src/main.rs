use std::net::{TcpStream};
use std::io::{Write};
fn main() -> ()
{
    let mut conn: TcpStream = TcpStream::connect("127.0.0.1:4545").expect("Failed!");
    let msg: &str = "Banana!!";
    let _ = conn.write(msg.as_bytes());
}

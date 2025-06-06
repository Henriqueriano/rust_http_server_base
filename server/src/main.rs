/* based on: https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html; */
use std::net::{TcpListener};
fn main() -> ()
{
    let listener = TcpListener::bind("127.0.0.1:4545").unwrap();
    println!("server_info: The rust socket listener in {:?}", TcpListener::local_addr(&listener));
            loop 
            {
                match listener.accept()
                {
                    Ok((_socket,addr)) => 
                    { 
                        println!("new client on {addr:?}"); 
                    },
                    Err(e) => 
                    {
                        println!("Could't get client: {e:?}");
                    }
                }
            }
    
        
}



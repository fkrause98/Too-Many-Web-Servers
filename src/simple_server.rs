use std::io;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
pub fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();
    loop {
        let (connection, _) = listener.accept().unwrap();
        if let Err(e) = handle_connection(connection) {
            println!("Handle connetion {e}");
        }
    }
}

fn handle_connection(mut connection: TcpStream) -> io::Result<()> {
    let mut request = [0u8; 1024];
    let mut read = 0;
    let mut written = 0;
    loop {
        let num_bytes = connection.read(&mut request[read..])?;
        if num_bytes == 0 {
            println!("Client disconnected unexpectedly");
            return Ok(());
        }
        read += num_bytes;
        if request.get(read - 4..read) == Some(b"\r\n\r\n") {
            break;
        }
    }
    let request = String::from_utf8_lossy(&request[..read]);
    println!("{request}");
    let response = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Length: 12\n",
        "Connection: close\r\n\r\n",
        "Hello world!"
    );
    loop {
        let num_bytes = connection.write(response[written..].as_bytes())?;
        if num_bytes == 0 {
            println!("client disconnected unexpectedly");
            return Ok(());
        }
        written += num_bytes;
        if written == response.len() {
            break;
        }
    }
    let _ = connection.flush();
    Ok(())
}

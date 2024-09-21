use std::io;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
pub fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut connections = Vec::new();
    loop {
        let connection = match listener.accept() {
            Ok((connection, _)) => {
                connection.set_nonblocking(true).unwrap();
                let state = ConnectionState::Read {
                    request: [0u8; 1024],
                    read: 0,
                };
                connections.push((connection, state));
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => panic!("{e}"),
        };
        let mut completed = Vec::new();
        'next: for (i, (connection, state)) in connections.iter_mut().enumerate() {
            match state {
                ConnectionState::Read { request, read } => loop {
                    match connection.read(&mut request[*read..]) {
                        Ok(0) => {
                            println!("Client disconnected prematurely!");
                            completed.push(i);
                            continue 'next;
                        }
                        Ok(n) => *read += n,
                        // not ready, continue to the next one
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue 'next;
                        }
                        Err(e) => panic!("Error reading socket: {e}"),
                    }
                    if request.get(*read - 4..*read) == Some(b"\r\n\r\n") {
                        let response = concat!(
                            "HTTP/1.1 200 OK\r\n",
                            "Content-Length: 12\n",
                            "Connection: close\r\n\r\n",
                            "Hello world!"
                        );

                        *state = ConnectionState::Write {
                            response: response.as_bytes(),
                            written: 0,
                        };
                        break;
                    }
                    let request = String::from_utf8_lossy(&request[..*read]);
                    println!("{request}");
                },
                ConnectionState::Write { response, written } => {
                    loop {
                        match connection.write(&response[*written..]) {
                            Ok(0) => {
                                println!("client disconnected unexpectedly");
                                completed.push(i);
                                continue 'next;
                            }
                            Ok(n) => {
                                *written += n;
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // not ready yet, move on to the next connection
                                continue 'next;
                            }
                            Err(e) => panic!("{e}"),
                        }

                        // did we write the whole response yet?
                        if *written == response.len() {
                            break;
                        }
                    }

                    // successfully wrote the response, try flushing next
                    *state = ConnectionState::Flush;
                }
                // Don't forget to flush!
                ConnectionState::Flush => match connection.flush() {
                    Ok(_) => {
                        completed.push(i);
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            continue 'next;
                        }
                    }
                    Err(e) => panic!("{e}"),
                },
            }
        }
        for i in completed.into_iter().rev() {
            connections.remove(i);
        }
    }
}

enum ConnectionState {
    Read {
        request: [u8; 1024],
        read: usize,
    },
    Write {
        response: &'static [u8],
        written: usize,
    },
    Flush,
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

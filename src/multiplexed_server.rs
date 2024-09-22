use std::{io::Write, net::TcpListener};
use polling::{Event, Events, Poller};
use smol::{io};

pub fn main() -> io::Result<()> {
    let response = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Length: 12\n",
        "Connection: close\r\n\r\n",
        "Hello world!"
    );
    smol::block_on(async {
        let mut listener = TcpListener::bind("localhost:3000").unwrap();
        listener.set_nonblocking(true)?;
        let poller = Poller::new()?;
        let key = 7;
        unsafe {
            poller.add(&listener, Event::readable(key)).unwrap();
        }
        let mut events = Events::new();
        loop {
            events.clear();
            poller.wait(&mut events, None)?;
            for ev in events.iter().filter(|&ev| ev.key == key) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        poller.modify(&listener, Event::readable(key))?;
                        stream.write(response.as_bytes())?;
                    }
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
                    err => return err
                }
            }
        }
    });
    return Ok(());
}

use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    println!("hello client");

    let socket_path = "/socket_file";

    let mut stream = UnixStream::connect(socket_path).expect("Connection is failed to server");
    stream
        .set_nonblocking(true)
        .expect("Couldn't set nonblocking");
    stream
        .set_read_timeout(Some(Duration::new(2, 0)))
        .expect("Couldn't set read timeout");

    stream.write_all(b"hello world")?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{response}");
    Ok(())
}

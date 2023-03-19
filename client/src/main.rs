use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::time::Duration;
use std::thread;

fn send_msg(stream: &mut UnixStream) -> std::io::Result<()> {
    stream.write(b"Hello server")?;

    Ok(())
}

fn read_from_stream(stream: &mut UnixStream) -> std::io::Result<()> {
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("We received this response: {}", response);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_path = "/socket_file";

    let mut stream = UnixStream::connect(socket_path).expect("Connection is failed to server");
    stream
        .set_nonblocking(false)
        .expect("Couldn't set nonblocking");
    stream
        .set_read_timeout(Some(Duration::new(10, 0)))
        .expect("Couldn't set read timeout");

    send_msg(&mut stream)?;
    thread::spawn(move || read_from_stream(&mut stream));
    Ok(())
}

use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::time::Duration;

fn write_request_and_shutdown(stream: &mut UnixStream) -> std::io::Result<()> {
    stream.write(b"Hello server")?;

    println!("We sent a request");
    println!("Shutting down writing on the stream, waiting for response...");

    // stream.shutdown(std::net::Shutdown::Write)?;

    Ok(())
}

fn read_from_stream(stream: &mut UnixStream) -> std::io::Result<()> {
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("We received this response: {}", response);
    Ok(())
}

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

    write_request_and_shutdown(&mut stream)?;
    read_from_stream(&mut stream)?;

    Ok(())
}

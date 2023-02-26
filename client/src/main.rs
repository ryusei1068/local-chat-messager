use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    println!("hello client");

    let socket_path = "/socket_file";

    let mut stream = UnixStream::connect(socket_path).expect("Failed connection to server");
    stream.write_all(b"hello world")?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{response}");
    Ok(())
}

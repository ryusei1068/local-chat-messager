use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    let mut message = String::new();
    stream.read_to_string(&mut message)?;

    println!("We received this message: {}\nReplying...", message);
    stream.write_all(b"hello client")?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_path = "/socket_file";

    let path = Path::new(&socket_path);

    if path.exists() {
        fs::remove_file(path).expect("File delete failed");
    }

    let listener = UnixListener::bind(socket_path)?;

    loop {
        let (mut unix_stream, socket_address) = listener.accept()?;
        println!("connection from {:?}", socket_address);
        thread::spawn(|| handle_client(unix_stream));
    }

    Ok(())
}

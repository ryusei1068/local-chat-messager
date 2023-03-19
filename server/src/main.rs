use std::fs;
use async_std::io::{WriteExt, ReadExt};
use async_std::os::unix::net::UnixListener;
use async_std::prelude::*;
use std::path::Path;
use async_std::task;

async fn run() -> std::io::Result<()> {

    let socket_path = "/socket_file";

    let path = Path::new(&socket_path);

    if path.exists() {
        fs::remove_file(path).expect("File delete failed");
    }

    let listener = UnixListener::bind(socket_path).await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        println!("connection from {:?}", stream.local_addr().unwrap());

        let mut message = String::new();
        stream.read_to_string(&mut message).await?;

        println!("We received this message: {}\nReplying...", message);
        
        stream.write_all(b"hello world").await?;
    }

    Ok(())
}

fn main() {
    task::block_on(run());
}

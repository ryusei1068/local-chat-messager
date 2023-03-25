use std::fs;
use async_std::io::{WriteExt, ReadExt};
use async_std::os::unix::net::UnixListener;
use async_std::prelude::*;
use std::path::Path;
use async_std::task;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;


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
        
        let name: String = Name(EN).fake(); 
        let mut greeting: String = "Hello, ".to_string();
        greeting.push_str(&name);
        let buf: &[u8] = greeting.as_str().as_bytes();
        
        stream.write_all(buf).await?;
    }

    Ok(())
}

fn main() {
    let result = task::block_on(run());
    println!("{:?}", result);
}

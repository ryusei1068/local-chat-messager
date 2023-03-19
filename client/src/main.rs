use async_std::io::{WriteExt, ReadExt};
use async_std::os::unix::net::UnixStream;
use std::net::Shutdown;
use async_std::task;

async fn say_request() -> std::io::Result<()> {
    let socket_path = "/socket_file";

    let mut stream = UnixStream::connect(socket_path).await.expect("failed connect to server");

    stream.write_all(b"Hello server").await?;

    stream.shutdown(Shutdown::Both).expect("shutdown function failed");

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    println!("We received this response: {}", response);

    Ok(())
}

fn main() {
    task::block_on(say_request());
}

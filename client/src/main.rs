use async_std::io::{WriteExt, ReadExt};
use async_std::os::unix::net::UnixStream;
use async_std::task;

async fn say_request() -> std::io::Result<String> {
    let socket_path = "/socket_file";

    let mut stream = UnixStream::connect(socket_path).await.expect("failed connect to server");

    stream.write_all(b"Hello server").await?;

    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    Ok(response)
}

fn main() {
    let response = task::block_on(say_request());
    println!("We received this response: {}", response.unwrap());
}

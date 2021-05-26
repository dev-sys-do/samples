use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:8080")?;
    client.write(b"GET / HTTP/1.1")?;

    let mut server_data = [0; 1024];
    client.read(&mut server_data)?;

    println!(
        "Server data:\n{}",
        String::from_utf8_lossy(&server_data[..])
    );

    Ok(())
}

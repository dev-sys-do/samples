use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let client = TcpStream::connect("127.0.0.1:8080")?;

    Ok(())
}

use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:8080")?;

    println!("Waiting for TCP clients...");
    for stream in server.incoming() {
        let stream = stream?;

        println!("New TCP client {:?}", stream.peer_addr()?);
    }

    Ok(())
}

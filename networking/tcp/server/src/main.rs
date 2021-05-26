use std::io::Read;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:8080")?;

    println!("Waiting for TCP clients...");
    for stream in server.incoming() {
        let mut stream = stream?;
        let mut client_data = [0; 1024];

        println!("New TCP client {:?}", stream.peer_addr()?);

        stream.read(&mut client_data)?;
        println!(
            "Client data:\n{}",
            String::from_utf8_lossy(&client_data[..])
        );
    }

    Ok(())
}

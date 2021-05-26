use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8083")?;
    let mut talker_data = [0; 10];

    socket.send_to(b"Hello", "127.0.0.1:8081")?;

    let (bytes, listener) = socket.recv_from(&mut talker_data)?;

    println!("Received {} bytes back from {:?}", bytes, listener);
    println!("Data:\n{}", String::from_utf8_lossy(&talker_data[..]));

    Ok(())
}

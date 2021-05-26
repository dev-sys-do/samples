use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8081")?;
    let mut talker_data = [0; 10];
    let (bytes, talker) = socket.recv_from(&mut talker_data)?;

    println!("Received {} bytes from {:?}", bytes, talker);
    println!("Data:\n{}", String::from_utf8_lossy(&talker_data[..]));

    socket.send_to(&mut talker_data[..bytes], &talker)?;

    Ok(())
}

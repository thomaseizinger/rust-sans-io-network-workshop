use anyhow::{bail, Result};
use sans_io_network_workshop::{Event, StunBinding};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::Instant,
};

const STUN1_L_GOOGLE_COM: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(172, 217, 192, 127)), 3478);

fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let local = socket.local_addr()?;
    let mut stun_binding = StunBinding::new(STUN1_L_GOOGLE_COM, Instant::now());

    let mut buf = [0u8; 1000];

    loop {
        let read_timeout = stun_binding.poll_timeout().map(|i| i - Instant::now());
        socket.set_read_timeout(read_timeout)?;

        let (num_bytes, from) = match socket.recv_from(&mut buf) {
            Ok((num_bytes, from)) => (num_bytes, from),
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
                ) =>
            {
                stun_binding.handle_timeout(Instant::now());
                continue;
            }
            Err(e) => bail!(e),
        };

        stun_binding.handle_input(from, local, &buf[..num_bytes], Instant::now());

        if let Some(Event::NewMappedAddress(addr)) = stun_binding.poll_event() {
            println!("New mapped address: {addr}")
        }
    }
}

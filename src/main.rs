use anyhow::{bail, Result};
use sans_io_network_workshop::{Event, StunBinding};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::Instant,
};

const FIREZONE_STUN_SERVER: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(35, 221, 210, 210)), 3478);

fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let local = socket.local_addr()?;
    let mut stun_binding = StunBinding::new(FIREZONE_STUN_SERVER, Instant::now());

    let mut buf = [0u8; 1000];

    loop {
        if let Some(transmit) = stun_binding.poll_transmit() {
            socket.send_to(&transmit.payload, transmit.dst)?;
            continue;
        }

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
            println!("Our public address is: {addr}");
            return Ok(());
        }
    }
}

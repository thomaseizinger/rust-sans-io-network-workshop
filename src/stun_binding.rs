use crate::{Event, Transmit};
use std::{net::SocketAddr, time::Instant};
use stun_codec::{
    rfc5389::{methods::BINDING, Attribute},
    Message, TransactionId,
};

/// A SANS-IO state machine that obtains the mapped address from the configured STUN server.
#[derive(Debug)]
pub struct StunBinding {}

impl StunBinding {
    pub fn new(server: SocketAddr, now: Instant) -> Self {
        Self {}
    }

    pub fn handle_input(
        &mut self,
        from: SocketAddr,
        local: SocketAddr,
        packet: &[u8],
        now: Instant,
    ) {
    }

    pub fn handle_timeout(&mut self, now: Instant) {}

    pub fn poll_event(&mut self) -> Option<Event> {
        None
    }

    pub fn poll_timeout(&mut self) -> Option<Instant> {
        None
    }

    pub fn poll_transmit(&mut self) -> Option<Transmit> {
        None
    }
}

fn new_stun_request() -> Message<Attribute> {
    Message::new(
        stun_codec::MessageClass::Request,
        BINDING,
        TransactionId::new(rand::random()),
    )
}

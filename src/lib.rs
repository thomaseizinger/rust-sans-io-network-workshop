mod stun_binding;

use bytecodec::{DecodeExt as _, EncodeExt as _};
use std::net::SocketAddr;
use stun_codec::{rfc5389::Attribute, DecodedMessage, Message};

pub use stun_binding::StunBinding;

#[derive(Debug)]
pub struct Transmit {
    pub dst: SocketAddr,
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    NewMappedAddress(SocketAddr),
}

pub fn encode(message: Message<Attribute>) -> Vec<u8> {
    stun_codec::MessageEncoder::default()
        .encode_into_bytes(message)
        .unwrap()
}

pub fn decode(bytes: &[u8]) -> bytecodec::Result<DecodedMessage<Attribute>> {
    stun_codec::MessageDecoder::default().decode_from_bytes(bytes)
}

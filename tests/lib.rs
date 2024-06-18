use sans_io_network_workshop::{decode, encode, Event, StunBinding, Transmit};
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::{Duration, Instant},
};
use stun_codec::{
    rfc5389::{attributes::XorMappedAddress, methods::BINDING, Attribute},
    Message, MessageClass,
};

const SERVER1: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3478));
const SERVER2: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 1), 3478));
const MAPPED_ADDRESS: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 9999));

#[test]
fn initial_binding_sends_request() {
    let mut stun_binding = StunBinding::new(SERVER1, Instant::now());

    let transmit = stun_binding.poll_transmit().unwrap();

    assert_eq!(transmit.dst, SERVER1);
}

#[test]
fn repeated_polling_does_not_generate_more_requests() {
    let mut stun_binding = StunBinding::new(SERVER1, Instant::now());

    assert!(stun_binding.poll_transmit().is_some());
    assert!(stun_binding.poll_transmit().is_none());
}

#[test]
fn mapped_address_is_emitted_as_event() {
    let mut stun_binding = StunBinding::new(SERVER1, Instant::now());

    let request = stun_binding.poll_transmit().unwrap();
    let response = generate_stun_response(request, MAPPED_ADDRESS);

    stun_binding.handle_input(SERVER1, MAPPED_ADDRESS, &response, Instant::now());

    let event = stun_binding.poll_event().unwrap();

    assert_eq!(event, Event::NewMappedAddress(MAPPED_ADDRESS));
}

fn generate_stun_response(request: Transmit, mapped_address: SocketAddr) -> Vec<u8> {
    let message = decode(&request.payload).unwrap().unwrap();

    let transaction_id = message.transaction_id();

    let mut response =
        Message::<Attribute>::new(MessageClass::SuccessResponse, BINDING, transaction_id);
    response.add_attribute(Attribute::XorMappedAddress(XorMappedAddress::new(
        mapped_address,
    )));

    encode(response)
}

extern crate libmpeg;

use libmpeg::*;

#[test]
fn null_packet_fields() {
    let packet = Packet::null();
    assert_eq!(false, packet.is_transport_error());
    assert_eq!(false, packet.is_payload_units_start());
    assert_eq!(false, packet.is_transport_priority());
    assert_eq!(0u32, packet.pid());
    assert_eq!(0u32, packet.transport_scrambling_control());
    assert_eq!(0u32, packet.continuity_counter());
    // TODO: check payload is zeroed
}

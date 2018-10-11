extern crate libmpeg;

use libmpeg::*;

#[test]
fn pid_zero() {
    let packet = Packet::from(([SYNC_BYTE, 0x00, 0x00, 0x00], [0xff; PAYLOAD_SIZE]));
    assert_eq!(0, packet.pid());
}

#[test]
fn pid_low_byte_only() {
    // Observation: if header[1] is odd, it adds 4096 to the pid
    // if even, it does nothing to the pid
    let packet = Packet::from(([SYNC_BYTE, 0x10, 0x00, 0x00], [0xff; PAYLOAD_SIZE]));
    assert_eq!(4096, packet.pid());
}

#[test]
fn pid_high_byte_only() {
    let packet = Packet::from(([SYNC_BYTE, 0x00, 0xff, 0x00], [0xff; PAYLOAD_SIZE]));
    assert_eq!(255, packet.pid());
}

#[test]
fn pid_max() {
    let packet = Packet::null();
    assert_eq!(PID_MAX, packet.pid());
}

#[test]
fn indexing() {
    let packet = Packet::null();
    assert_eq!(0x47, packet[0]);
    assert_eq!(0x1f, packet[1]);
    assert_eq!(0xff, packet[2]);
    assert_eq!(0x00, packet[3]);
    assert_eq!(0xff, packet[4]);
    assert_eq!(0xff, packet[104]);
    assert_eq!(0xff, packet[187]);
}

#[test]
#[should_panic]
fn indexing_out_of_bounds() {
    let packet = Packet::null();
    let _ = packet[188];
}

#[test]
fn identical_packets_are_equal() {
    let p1: Packet = Default::default();
    let p2: Packet = Default::default();
    assert_eq!(p1, p2);
}

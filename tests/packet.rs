extern crate libmpeg;

use libmpeg::*;

#[test]
fn default_has_max_pid() {
    let packet = Packet::default();
    assert_eq!(MAX_PID, packet.pid());
}

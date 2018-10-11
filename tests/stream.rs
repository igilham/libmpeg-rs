extern crate libmpeg;
use libmpeg::*;

// TODO: figure out packet reader test
// #[test]
// fn read_sample_packet() {
//     let expected = Packet::default();
//     let buf: PacketBuffer = Packet::default().into();
//     let mut stream = Stream::from(&buf[0..188]);
//     let result = stream.read_packet();
//     match result {
//         Ok(packet) => assert_eq!(expected.pid(), packet.pid()),
//         Err(_) => assert!(false),
//     }
// }

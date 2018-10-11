extern crate libmpeg;
use libmpeg::*;

#[test]
fn pull_byte() {
    let data: [u8; 6] = [0x1, 0x1, 0x2, 0x3, 0x5, 0x8];
    let mut stream = Stream::from(&data[..]);

    assert_eq!(stream.pull_byte().unwrap(), 0x1);
    assert_eq!(stream.pull_byte().unwrap(), 0x1);
    assert_eq!(stream.pull_byte().unwrap(), 0x2);
    assert_eq!(stream.pull_byte().unwrap(), 0x3);
    assert_eq!(stream.pull_byte().unwrap(), 0x5);
    assert_eq!(stream.pull_byte().unwrap(), 0x8);
}

#[test]
fn pull_bit() {
    let data: [u8; 2] = [0b10010110, 0b10100101];
    let mut stream = Stream::from(&data[..]);

    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
}

#[test]
fn pull_bits() {
    let data: [u8; 2] = [0b10101010, 0b10010011];
    let mut stream = Stream::from(&data[..]);

    assert_eq!(stream.pull_bits(2).unwrap(), 0b10);
    assert_eq!(stream.pull_bits(3).unwrap(), 0b010);
    assert_eq!(stream.pull_bits(3).unwrap(), 0b101);
    assert_eq!(stream.pull_bits(4).unwrap(), 0b0011);
    assert_eq!(stream.pull_bits(1).unwrap(), 0b1);
    assert_eq!(stream.pull_bits(3).unwrap(), 0b100);
}

#[test]
fn pull_bits_u16() {
    let data: [u8; 3] = [0b10010011, 0b10101010, 0b11110000];
    let mut stream = Stream::from(&data[..]);

    assert_eq!(stream.pull_bits_u16(9).unwrap(), 0b100100110);
    assert_eq!(stream.pull_bits_u16(12).unwrap(), 0b101010110000);
    assert_eq!(stream.pull_bits(3).unwrap(), 0b111);
}

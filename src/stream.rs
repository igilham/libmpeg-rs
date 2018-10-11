// MPEG Transport Stream
use super::packet::*;
use std::io;
// use std::fmt;
// use std::ops;

pub struct Stream<R: io::Read> {
      source: R,
}

impl<R: io::Read> Stream<R> {
    pub fn read_packet(&mut self) -> Result<Packet, io::Error> {
        let mut buf = [0u8; PACKET_SIZE];
        self.source.read(&mut buf)?;
        Ok(Packet::from(buf))
    }
}

impl<R: io::Read> From<R> for Stream<R> {
    fn from(r: R) -> Self {
        Stream { source: r }
    }
}

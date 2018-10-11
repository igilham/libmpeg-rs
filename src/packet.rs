use std::fmt;
use super::stream::*;

pub const PACKET_SIZE: usize = 188;
pub const PAYLOAD_SIZE: usize = 184;
pub const SYNC_BYTE: u8 = 0x47;
pub const MAX_PID: u16 = 0x1FFF;

#[derive(Eq, PartialEq, Clone)]
pub struct Packet {
    transport_error: bool,
    payload_unit_start: bool,
    transport_priority: bool,
    pid: u16,
    transport_scrambling_control: u8,
    continuity_counter: u8,
    adaptation_field: Option<AdaptationField>,
    payload: Option<Vec<u8>>,
}

// TODO: fill out adaptation field
#[derive(Eq, PartialEq, Clone)]
pub struct AdaptationField{}

impl Packet {
    pub fn null() -> Self {
        Packet {
            transport_error: false,
            payload_unit_start: false,
            transport_priority: false,
            pid: MAX_PID,
            transport_scrambling_control: 0,
            continuity_counter: 0,
            adaptation_field: None,
            payload: Some(vec!(0xffu8; PAYLOAD_SIZE)),
        }
    }

    pub fn parse(data: &[u8]) -> Result<Packet, &'static str> {
        let mut s = Stream::from(data);
        let sync_byte: u8 = s.pull_byte()?;
        if sync_byte != SYNC_BYTE {
            return Err("Invalid sync byte");
        }

        let mut packet = Packet::default();
        packet.transport_error = s.pull_bit()?;
        packet.payload_unit_start = s.pull_bit()?;
        packet.transport_priority = s.pull_bit()?;
        packet.pid = s.pull_bits_u16(13)?;
        packet.transport_scrambling_control = s.pull_bits(2)?;
        let adaptation_field_flag = s.pull_bit()?;
        let payload_flag = s.pull_bit()?;
        packet.continuity_counter = s.pull_bits(4)?;
        packet.payload = None;

        // TODO: build adaptation field
        if adaptation_field_flag {}

        Ok(packet)
    }

    pub fn transport_error(&self) -> bool {
        self.transport_error
    }

    pub fn payload_unit_start(&self) -> bool {
        self.payload_unit_start
    }

    pub fn transport_priority(&self) -> bool {
        self.transport_priority
    }

    pub fn pid(&self) -> u16 {
        self.pid
    }

    pub fn transport_scrambling_control(&self) -> u8 {
        self.transport_scrambling_control
    }

    pub fn continuity_counter(&self) -> u8 {
        self.continuity_counter
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( pid: {} )", self.pid())
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self::null()
    }
}

impl<'a> From<&'a[u8]> for Packet {
    // Panics if parsing fails
    fn from(data: &[u8]) -> Self {
        Packet::parse(data).unwrap()
    }
}

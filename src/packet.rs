// packet
use std::fmt;

const SYNC_BYTE: u8 = 0x47;
const PACKET_SIZE: usize = 188;
const MAX_PAYLOAD_SIZE: usize = 184;

pub const PID_MAX: u16 = 8191;

pub struct Packet {
    transport_error_indicator: bool,
    payload_units_start_indicator: bool,
    transport_priority: bool,
    pid: u32,
    transport_scrambling_control: u32,
    // adaptation_field_control: AdaptationFieldControl,
    continuity_counter: u32,
    // adaptation_field: AdaptationField,
    // payload: [u8],
    payload_buffer: [u8; MAX_PAYLOAD_SIZE],
}

impl Packet {
    pub fn null() -> Self {
        Packet {
            transport_error_indicator: false,
            payload_units_start_indicator: false,
            transport_priority: false,
            pid: 0,
            transport_scrambling_control: 0,
            // adaptation_field_control: nil,
            continuity_counter: 0,
            // adaptation_field: AdaptationField,
            // payload: [u8],
            payload_buffer: [0u8; MAX_PAYLOAD_SIZE],
        }
    }

    pub fn is_transport_error(&self) -> bool {
        self.transport_error_indicator
    }

    pub fn is_payload_units_start(&self) -> bool {
        self.payload_units_start_indicator
    }

    pub fn is_transport_priority(&self) -> bool {
        self.transport_priority
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn transport_scrambling_control(&self) -> u32 {
        self.transport_scrambling_control
    }

    pub fn continuity_counter(&self) -> u32 {
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

impl From<[u8; PACKET_SIZE]> for Packet {
    fn from(data: [u8; 188]) -> Self {
        // TODO: implement correctly
        Packet::default()
    }
}

impl Into<[u8; PACKET_SIZE]> for Packet {
    fn into(self) -> [u8;188] {
        // TODO: implement correctly
        [0u8; PACKET_SIZE]
    }
}

impl PartialEq for Packet {
    // Compare packets by pid only
    fn eq(&self, other: &Self) -> bool {
        self.pid() == other.pid()
    }
}

impl Eq for Packet {}

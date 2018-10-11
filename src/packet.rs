// MPEG Transport Stream Packet
use std::fmt;
use std::ops;

pub const SYNC_BYTE: u8 = 0x47;
pub const HEADER_SIZE: usize = 4;
pub const PACKET_SIZE: usize = 188;
pub const PAYLOAD_SIZE: usize = PACKET_SIZE - HEADER_SIZE;

pub const PID_MAX: u16 = 8191;

pub type Header = [u8; HEADER_SIZE];

pub type Payload = [u8; PAYLOAD_SIZE];

pub type PacketBuffer = [u8; PACKET_SIZE];

// Allocate a payload. Default::default does not work here
fn allocate_payload() -> Payload {
    [0u8; PAYLOAD_SIZE]
}

#[derive(Clone)]
pub struct Packet {
    header: Header,
    payload: Payload,
}

impl Packet {
    pub fn null() -> Self {
        return Self {
            header: [SYNC_BYTE, 0x1f, 0xff, 0x00],
            payload: [0xff; PAYLOAD_SIZE],
        }
    }

    pub fn pid(&self) -> u16 {
        return (((self.header[1] & 0x1f) as u16) << 8) +
                self.header[2] as u16;
    }

    pub fn has_pcr(&self) -> bool {
        return (self.has_adaptation_field()) &&
                (self.payload[0] != 0) &&
                (self.payload[1] & 0x10 != 0);
    }

    pub fn read_pcr(&self) -> u64 {
        if !self.has_pcr() {
            return 0;
        }
        let mut base: u64 = 0;
        let mut ext: u32 = 0;

        base |= (self.payload[2] as u64) << 25;
        base |= (self.payload[3] as u64) << 17;
        base |= (self.payload[4] as u64) << 9;
        base |= (self.payload[5] as u64) << 1;
        base |= ((self.payload[6] & 0x80) as u64) >> 7;

        ext |= ((self.payload[6] & 0x01) as u32) << 8;
        ext |= self.payload[7] as u32;

        return base * 300 + (ext as u64);
    }

    pub fn write_pcr(&mut self, pcr: u64) -> bool {
        if !self.has_pcr() {
            return false;
        }

        let base: u64 = pcr / 300;
        let ext: u32 = (pcr % 300) as u32;

        self.payload[7] = (ext & 0xff) as u8;
        self.payload[6] = 0x7e;	// reserved bits
        self.payload[6] |= ((ext & 0x100) >> 8) as u8;

        self.payload[6] |= ((base & 0x01) << 7) as u8;
        self.payload[5] = ((base >> 1) & 0xff) as u8;
        self.payload[4] = ((base >> 9) & 0xff) as u8;
        self.payload[3] = ((base >> 17) & 0xff) as u8;
        self.payload[2] = ((base >> 25) & 0xff) as u8;

        return true;
    }

    pub fn set_discontinuity(&mut self) -> bool {
        if (self.has_adaptation_field()) && (self.payload[0] != 0) {
            // set discontinuity_indicator bit
            self.payload[1] |= 0x80;
            return true;
        }
        return false;
    }

    pub fn has_dts(&self) -> bool {
        if !self.has_payload_unit_start_indicator() || !self.has_payload() {
            return false;
        }

        let pes_start = self.pes_start();
        if pes_start > 184 - 19 {
            // not enough data to ensure we can get DTS out
            return false;
        }
        if self.payload[pes_start] != 0 ||
                self.payload[pes_start+1] != 0 ||
                self.payload[pes_start+2] != 1 {
            // error -- bad PES packet
            return false;
        }

        let stream_id: u8 = self.payload[pes_start+3];
        if (stream_id & 0xe0) != 0xc0 && (stream_id & 0xf0) != 0xe0 {
            // not audio or video (should check some others too)
            return false;
        }

        // PTS or DTS
        return self.payload[pes_start+7] & 0x80 == 0;
    }

    pub fn read_dts(&self) -> u64 {
        // check that the packet has a DTS
        if !self.has_dts() {
            return 0;
        }

        let pes_start = self.pes_start();
        let pts_dts_flags: u8 = self.payload[pes_start+7] & 0xc0;

        // PTS only: return it
        let mut dts_index: usize = pes_start + 9;

        if pts_dts_flags != 0x80 {
            // DTS
            dts_index = pes_start + 14;
        }

        let mut dts: u64 = 0;
        dts |= ((self.payload[dts_index] & 0x0e) as u64) << 29;
        dts |= ((self.payload[dts_index+1]) as u64) << 22;
        dts |= ((self.payload[dts_index+2] & 0xfe) as u64) << 14;
        dts |= ((self.payload[dts_index+3]) as u64) << 7;
        dts |= ((self.payload[dts_index+4] & 0xfe) as u64) >> 1;

        return dts * 300;
    }

    fn has_adaptation_field(&self) -> bool {
        return self.header[3] & 0x20 != 0;
    }

    fn has_payload_unit_start_indicator(&self) -> bool {
        return self.header[1] & 0x40 != 0;
    }

    fn has_payload(&self) -> bool {
        return self.header[3] & 0x10 != 0;
    }

    // Get the start position of the Packetised Elementary Stream
    fn pes_start(&self) -> usize {
        let mut index: usize = 0;
        if self.has_adaptation_field() {
            index += self.payload[0] as usize + 1;
        }
        return index;
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

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..PACKET_SIZE {
            if self[i] != other[i] {
                return false;
            }
        }
        return true;
    }
}

impl Eq for Packet {}

// convert from fixed-size array of u8 ([u8; PACKET_LEN])
impl From<PacketBuffer> for Packet {
    fn from(data: PacketBuffer) -> Self {
        let mut header: Header = Default::default();
        let mut payload: Payload = allocate_payload();

        header.copy_from_slice(&data[0 .. HEADER_SIZE]);
        payload.copy_from_slice(&data[HEADER_SIZE .. PAYLOAD_SIZE]);

        return Self::from((header, payload));
    }
}

// Convert from a tuple of fixed-size arrays
impl From<(Header, Payload)> for Packet {
    fn from(data: (Header, Payload)) -> Self {
        return Self { header: data.0, payload: data.1 };
    }
}

// Convert to an array
impl Into<PacketBuffer> for Packet {
    fn into(self) -> PacketBuffer {
        let mut buf: PacketBuffer = [0u8; PACKET_SIZE];
        for i in 0 .. PACKET_SIZE {
            buf[i] = self[i]
        }
        buf
    }
}

// Allow indexing TSPacket like an array spanning header and paylaod
impl ops::Index<usize> for Packet {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        if index < HEADER_SIZE {
            &self.header[index]
        } else {
            &self.payload[index - HEADER_SIZE]
        }
    }
}



// MPEG Transport Stream

pub struct Stream<'a> {
      data: &'a[u8],
      position: usize,
      bit_position: u8,
}

impl<'a> Stream<'a> {
    // Read a byte from the stream
    pub fn pull_byte(&mut self) -> Result<u8, &'static str> {
        if self.bit_position > 0 {
            Err("Requested byte, but bits have already been pulled from the current byte")
        } else if self.position >= self.data.len() {
            Err("No data remaining")
        } else {
            let v = self.data[self.position];
            self.position += 1;
            Ok(v)
        }
    }

    /// Pull a single bit from the stream
    pub fn pull_bit(&mut self) -> Result<bool, &'static str> {
        if self.position >= self.data.len() {
            Err("No data remaining")
        } else {
            let v = (self.data[self.position] & (1 << self.bit_position)) > 0;

            if self.bit_position == 7 {
                self.position += 1;
                self.bit_position = 0;
            } else {
                self.bit_position += 1;
            }
            Ok(v)
        }
    }

    /// Pull n bits from the stream (from current byte position only)
    /// Cannot pull more than 8 bits
    pub fn pull_bits(&mut self, n: u8) -> Result<u8, &'static str> {
        if n == 8 {
            self.pull_byte()
        } else if self.bit_position + n > 8 {
            Err("Requested more bits than what remains in the current byte")
        } else {
            // Bit twiddling ahead! It's dangerous to go alone, take these notes.
            // Produce a mask to extract the desired bits from the current marked position
            // Example:
            // bit_marker = 00000100, bit_position = 3, byte = 01001110, n = 3
            //
            // First get mask for unwanted least significant bits:
            //      bit_marker - 1 = 00000011
            // Next get mask for unwanted most siginifcant bits:
            //      bit_marker << n = 00100000
            //      00100000 - 1 = 00011111
            //      !00011111 = 11100000
            // Next combine the two results to get the mask for unwanted bits
            //        00000011
            //      | 11100000
            //      = 11100011
            // Finally, obtain the mask
            //      !11100011 = 00011100
            // This mask can be used extract n bits from the current byte
            //      byte | 00011100 = 00011000
            //      00011000 >> bit_position = 00000011
            let marker = 1 << self.bit_position;
            let mask = !((marker - 1) | !(((marker as u16) << n) - 1) as u8);
            let v = (self.data[self.position] & mask) >> self.bit_position;

            if self.bit_position + n == 8 {
                self.position += 1;
                self.bit_position = 0;
            } else {
                self.bit_position += n;
            }
            Ok(v)
        }
    }

    /// Assumes BE at the moment, which is how MPEG-TS packs its bytes
    pub fn pull_bits_u16(&mut self, n: u8) -> Result<u16, &'static str> {
        if n > 16 {
            Err("Requested more than what exists in a u16")
        } else {
            let n1 = 8 - self.bit_position;
            let n2 = n - n1;
            Ok((try!(self.pull_bits(n1)) as u16) << n2 | try!(self.pull_bits(n2)) as u16)
        }
    }
}

impl<'a> From<&'a[u8]> for Stream<'a> {
    fn from(data: &'a[u8]) -> Stream<'a> {
        Stream {
            data: data,
            position: 0,
            bit_position: 0,
        }
    }
}

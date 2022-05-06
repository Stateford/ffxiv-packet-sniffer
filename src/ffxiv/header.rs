use std::fmt::Display;

pub struct FFXIVPacketHeader {
    pub magic: u128,
    pub epoch: u64,
    pub length: u16,
    pub unknown1: u16,
    pub connection_type: u16,
    pub message_count: u16,
    pub encoding: u8,
    pub compressed: u8,
    pub unknown2: u16,
    pub unknown3: u16,
    pub unknown4: u16,
}

impl FFXIVPacketHeader {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            magic: u128::from_le_bytes(bytes[0..16].try_into().unwrap()),
            epoch: u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
            length: u16::from_le_bytes(bytes[24..26].try_into().unwrap()),
            unknown1: u16::from_le_bytes(bytes[26..28].try_into().unwrap()),
            connection_type: u16::from_le_bytes(bytes[28..30].try_into().unwrap()),
            message_count: u16::from_le_bytes(bytes[30..32].try_into().unwrap()),
            encoding: u8::from_le_bytes(bytes[32..33].try_into().unwrap()),
            compressed: u8::from_le_bytes(bytes[33..34].try_into().unwrap()),
            unknown2: u16::from_le_bytes(bytes[34..36].try_into().unwrap()),
            unknown3: u16::from_le_bytes(bytes[36..38].try_into().unwrap()),
            unknown4: u16::from_le_bytes(bytes[38..40].try_into().unwrap()),
        }
    }
}

impl Display for FFXIVPacketHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t Magic: {:2X}", self.magic)?;
        writeln!(f, "\t Epoch: {}", self.epoch)?;
        writeln!(f, "\t Length: {}", self.length)?;
        writeln!(f, "\t Unknown1: {}", self.unknown1)?;
        writeln!(f, "\t Connection Type: {}", self.connection_type)?;
        writeln!(f, "\t Message Count: {}", self.message_count)?;
        writeln!(f, "\t Encoding: {}", self.encoding)?;
        writeln!(f, "\t Compressed: {}", self.compressed)?;
        writeln!(f, "\t Unknown2: {}", self.unknown2)?;
        writeln!(f, "\t Unknown3: {}", self.unknown3)?;

        Ok(())
    }
}

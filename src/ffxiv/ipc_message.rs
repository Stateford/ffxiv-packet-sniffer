use std::fmt::Display;

pub struct FFXIVIpcMessage {
    pub magic: u16,
    pub ipc_type: u16,
    pub unknown1: u16,
    pub server_id: u16,
    pub epoch: u32,
    pub data: Vec<u8>,
}

impl FFXIVIpcMessage {
    pub fn new(data: &[u8]) -> Self {
        Self {
            magic: u16::from_le_bytes(data[..2].try_into().unwrap()),
            ipc_type: u16::from_le_bytes(data[2..4].try_into().unwrap()),
            unknown1: u16::from_le_bytes(data[4..6].try_into().unwrap()),
            server_id: u16::from_le_bytes(data[6..8].try_into().unwrap()),
            epoch: u32::from_le_bytes(data[8..12].try_into().unwrap()),
            data: data[12..].to_vec(),
        }
    }
}

impl Display for FFXIVIpcMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t Magic: {:2X}", self.magic)?;
        writeln!(f, "\t IPC Type: {:2X}", self.ipc_type)?;
        writeln!(f, "\t Unknown1: {:2X}", self.unknown1)?;
        writeln!(f, "\t Server ID: {:2X}", self.server_id)?;
        writeln!(f, "\t Epoch: {}", self.epoch)?;
        writeln!(f, "\t Data: {:?}", self.data)?;

        Ok(())
    }
}
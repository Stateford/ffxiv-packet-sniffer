use std::fmt::Display;
use crate::ffxiv::ipc_message::FFXIVIpcMessage;

pub struct FFXIVPacketMessage {
    pub message_length: u32,
    pub source: u32,
    pub target: u32,
    pub message_type: u16,
    pub unknown1: u16,
    pub ipc_message: FFXIVIpcMessage,
}

impl FFXIVPacketMessage {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            message_length: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            source: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            target: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            message_type: u16::from_le_bytes(bytes[12..14].try_into().unwrap()),
            unknown1: u16::from_le_bytes(bytes[14..16].try_into().unwrap()),
            ipc_message: FFXIVIpcMessage::new(&bytes[16..]),
        }
    }
}

impl Display for FFXIVPacketMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t Message Length: {:2X}", self.message_length)?;
        writeln!(f, "\t Source ID: {:2X}", self.source)?;
        writeln!(f, "\t Target: {:2X}", self.target)?;
        writeln!(f, "\t Message Type: {:2X}", self.message_type)?;
        writeln!(f, "\t Unknown1: {:2X}", self.unknown1)?;
        writeln!(f, "\t IPC Data:\n{}", self.ipc_message)?;

        Ok(())
    }
}
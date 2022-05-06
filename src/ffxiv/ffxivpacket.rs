use flate2::read::ZlibDecoder;
use std::fmt::Display;
use std::io::prelude::*;
use crate::error::PacketSnifferError;
use crate::ffxiv::header::FFXIVPacketHeader;
use crate::ffxiv::message::FFXIVPacketMessage;

const FFXIV_HEADER_SIZE: usize = 40;


pub struct FFXIVPacket {
    header: FFXIVPacketHeader,
    messages: Vec<FFXIVPacketMessage>,
}

impl FFXIVPacket {
    pub fn new(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if data.len() < 40 || data[0] == 0x0 {
            return Err(Box::new(PacketSnifferError));
        }
        let header = FFXIVPacketHeader::new(&data[0..FFXIV_HEADER_SIZE]);
        let mut messages = Vec::new();
        let mut decompressed_message = Vec::<u8>::new();
        let message = if header.compressed == 1 {
            let mut gz = ZlibDecoder::new(&data[FFXIV_HEADER_SIZE..]);
            gz.read_to_end(&mut decompressed_message).unwrap();
            &decompressed_message
        } else {
            &data[FFXIV_HEADER_SIZE..]
        };

        let mut offset = 0;
        for _ in 0..header.message_count {
            let message_size = u32::from_le_bytes(message[offset..offset + 4].try_into().unwrap());
            messages.push(FFXIVPacketMessage::new(
                &message[offset..offset + message_size as usize],
            ));
            offset += message_size as usize;
        }
        Ok(Self { header, messages })
    }

    #[allow(dead_code)]
    pub fn header(&self) -> &FFXIVPacketHeader {
        &self.header
    }

    pub fn messages(&self) -> &Vec<FFXIVPacketMessage> {
        &self.messages
    }
}

impl Display for FFXIVPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HEADER:")?;
        write!(f, "{}", self.header)?;

        writeln!(f, "Message:")?;
        for message in &self.messages {
            write!(f, "{}", message)?;
        }

        Ok(())
    }
}

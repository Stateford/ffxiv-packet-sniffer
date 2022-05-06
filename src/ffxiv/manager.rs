use crate::packetsniffer::PacketSniffer;
use super::ffxivpacket::FFXIVPacket;

const FFXIV_PROCESS_NAME: &str = "ffxiv_dx11.exe";

pub struct FFXIVPacketManager {
    packet_sniffer: PacketSniffer
}

impl FFXIVPacketManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            packet_sniffer: PacketSniffer::new(FFXIV_PROCESS_NAME)?
        })
    }

    pub fn get_packets(&mut self) -> Option<Vec<FFXIVPacket>> {
        let mut ffxiv_packets = Vec::new();
        let packets = self.packet_sniffer.recv();

        if packets.is_empty() {
            return None;
        }

        for packet in packets {
            if let Ok(ffxiv_packet) = FFXIVPacket::new(&packet) {
                ffxiv_packets.push(ffxiv_packet);
            }
        }

        Some(ffxiv_packets)
    }
}

impl Default for FFXIVPacketManager {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
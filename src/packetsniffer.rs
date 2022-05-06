use crate::sockets::Sockets;
use pnet::datalink::{interfaces, NetworkInterface};
use pnet::ipnetwork::IpNetwork;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::{
    datalink,
    datalink::{Channel::Ethernet, DataLinkReceiver, DataLinkSender},
    packet::Packet,
};
use std::net::IpAddr;

#[allow(dead_code)]
pub struct PacketSniffer {
    sockets: Sockets,
    tx: Box<dyn DataLinkSender>,
    rx: Box<dyn DataLinkReceiver>,
}

impl PacketSniffer {
    fn default_interface() -> Option<NetworkInterface> {
        interfaces().into_iter().find(|e| {
            !e.ips
                .contains(&IpNetwork::new("0.0.0.0".parse().unwrap(), 0).unwrap())
                && !e.description.contains("Virtual")
        })
    }
}

impl PacketSniffer {
    pub fn new(process_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let interface = Self::default_interface().unwrap();

        println!("interface: {}", interface.description);

        let (tx, rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("UNHANDLED CHANNEL TYPE"),
            Err(_) => panic!("an error occurred creating data links"),
        };
        let mut sockets = Sockets::get_process_sockets(process_name)?;
        sockets.gather_associated_ips();
        Ok(Self { sockets, rx, tx })
    }

    pub fn recv(&mut self) -> Vec<Vec<u8>> {
        let mut packets: Vec<Vec<u8>> = Vec::new();

        if let Ok(packet) = self.rx.next() {
            let packet = EthernetPacket::new(packet).unwrap();
            let header = Ipv4Packet::new(packet.payload()).unwrap();

            let source = IpAddr::V4(header.get_source());
            let destination = IpAddr::V4(header.get_destination());

            if self.sockets.ip_addresses.contains(&source)
                || self.sockets.ip_addresses.contains(&destination)
            {
                let tcp = TcpPacket::new(header.payload()).unwrap();
                if !tcp.payload().is_empty() {
                    packets.push(tcp.payload().to_vec());
                }
            }
        }

        packets
    }
}

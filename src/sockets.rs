use crate::error::PacketSnifferError;
use netstat2::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo,
};
use std::fmt::Display;
use std::net::IpAddr;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

pub struct Sockets {
    pub sockets: Vec<SocketInfo>,
    pub ip_addresses: Vec<IpAddr>,
}

impl Sockets {
    pub fn get_process_sockets(process_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sys = System::new_all();

        let sockets_info = get_sockets_info(
            AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
            ProtocolFlags::TCP | ProtocolFlags::UDP,
        )?;

        let process = sys
            .processes_by_exact_name(process_name)
            .next()
            .ok_or(PacketSnifferError)?;

        let sockets: Vec<SocketInfo> = sockets_info
            .into_iter()
            .filter(|si| si.associated_pids.contains(&process.pid().as_u32()))
            .collect();

        Ok(Self {
            sockets,
            ip_addresses: Vec::new(),
        })
    }

    pub fn gather_associated_ips(&mut self) {
        self.ip_addresses.clear();

        for socket in &self.sockets {
            if let ProtocolSocketInfo::Tcp(tcp_si) = &socket.protocol_socket_info {
                self.ip_addresses.push(tcp_si.remote_addr);
            }
        }
        println!("{:?}", self.ip_addresses);
    }
}

impl Display for Sockets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for socket in &self.sockets {
            match &socket.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp_si) => {
                    writeln!(
                        f,
                        "TCP {}:{} -> {}:{} {:?} - {}",
                        tcp_si.local_addr,
                        tcp_si.local_port,
                        tcp_si.remote_addr,
                        tcp_si.remote_port,
                        socket.associated_pids,
                        tcp_si.state
                    )?;
                }
                ProtocolSocketInfo::Udp(udp_si) => {
                    writeln!(
                        f,
                        "UDP {}:{} -> *:* {:?}",
                        udp_si.local_addr, udp_si.local_port, socket.associated_pids
                    )?;
                }
            }
        }
        Ok(())
    }
}

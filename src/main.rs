extern crate flate2;
extern crate log;
extern crate netstat2;
extern crate pnet;
extern crate sysinfo;

mod error;
mod ffxiv;
mod packetsniffer;
mod sockets;

use std::time::Duration;
use ffxiv::manager::FFXIVPacketManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FFXIVPacketManager::new()?;

    let mut running = true;
    while running {
        if let Some(packets) = client.get_packets() {
            for packet in packets {
                println!("{}", packet);
                let message_types: Vec<u16> = packet
                    .messages()
                    .iter()
                    .map(|x| x.ipc_message.ipc_type)
                    .collect();
                if message_types.contains(&0x104) || message_types.contains(&0x22F) {
                    println!("{}", packet);
                    running = false;
                    break;
                }
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

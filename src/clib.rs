#![feature(once_cell)]
use ffxiv::client::FFXIVClient;
use std::sync::Mutex;
use std::lazy::SyncLazy;


static FFXIV_CLIENT: SyncLazy<Mutex<FFXIVClient>> = SyncLazy::new(|| Mutex::new(FFXIVClient::new()));

#[repr(C)]
#[derive(Debug)]
enum FFXIVErrors {
    SUCCESS = 0,
    ERROR = -1
}

#[no_mangle]
pub extern "C" fn get_packet() -> i32 {
    let mut client = FFXIV_CLIENT.lock().unwrap();
    match client.get_packets() {
        Some(packet) => {
            FFXIVErrors::SUCCESS as i32
        },
        None => FFXIVErrors::ERROR as i32
    }
}
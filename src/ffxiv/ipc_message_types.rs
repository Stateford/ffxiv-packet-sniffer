struct IpcPlayerPosition {
    number: u32
}

struct IpcCharacter {
    title: u32,
    current_server: u16,
    original_server: u16,
    unknown1: u32,
    unknown2: u32,
    target_id: u32,
    npc_base: u32,
    npc_id: u16,
    chracter_type: u16,
    owner_id: u32,
    nickname: String
}

struct IpcPlayerStats {
    
}

enum IpcMessageTypes<'a> {
    UpdatePlayerPosition(IpcPlayerPosition),
    Character(IpcCharacter),
    Unsupported(&'a [u8])
}
#[derive(Debug, Clone)]
pub struct PacketSnifferError;

impl std::fmt::Display for PacketSnifferError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Error occurred")
    }
}

impl std::error::Error for PacketSnifferError {}

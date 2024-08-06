pub fn chunk_file(file_data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
    file_data
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

pub fn unchunk_file(chunks: Vec<Vec<u8>>) -> Vec<u8> {
    chunks.into_iter().flatten().collect()
}

pub enum TransferProtocol {
    Tcp,
    Udp,
}

impl TransferProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            TransferProtocol::Tcp => "tcp",
            TransferProtocol::Udp => "udp",
        }
    }
}

pub struct FileChunk {
    pub id: u32,
    pub data: Vec<u8>,
}

pub trait Transfer {
    fn send(&self, data: &[u8]) -> Result<(), String>;
    fn receive(&self) -> Result<Vec<u8>, String>;
}
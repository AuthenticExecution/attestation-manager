#[derive(Debug, Clone)]
pub struct Module {
    pub id : u16,
    pub host : String,
    pub port : u16,
    pub em_port : u16,
    pub key : Vec<u8>
}

impl Module {
    pub fn new(id : u16, host : String, port : u16, em_port : u16, key : Vec<u8>) -> Module {
        Module {
            id,
            host,
            port,
            em_port,
            key
        }
    }
}

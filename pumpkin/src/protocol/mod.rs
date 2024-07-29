use bytebuf::buffer::ByteBuffer;
use serde::{Deserialize, Serialize};

pub mod bytebuf;

pub mod client;
pub mod server;

type VarInt = i32;
type VarLong = i64;

#[derive(Debug)]
pub enum ConnectionState {
    HandShake,
    Status,
    Login,
    Transfer,
}

impl ConnectionState {
    pub fn from_varint(var_int: VarInt) -> Self {
        match var_int {
            1 => Self::Status,
            2 => Self::Login,
            3 => Self::Transfer,
            _ => panic!("Unexpected Status {}", var_int),
        }
    }
}

#[derive(Debug)]
pub struct RawPacket {
    pub len: VarInt,
    pub id: VarInt,
    pub bytebuf: ByteBuffer,
}

pub trait ClientPacket {
    const PACKET_ID: VarInt;

    fn write(&self, bytebuf: &mut ByteBuffer);
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    pub version: Version,
    pub players: Players,
    pub description: String,
    pub favicon: String, // data:image/png;base64,<data>
                         // Players, favicon ...
}
#[derive(Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    pub max: u32,
    pub online: u32,
    pub sample: Sample,
}

#[derive(Serialize, Deserialize)]
pub struct Sample {
    pub name: String,
    pub id: String, // uuid
}

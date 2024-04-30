//! Serialization and deserialization of types

pub mod bitcode;
pub mod reader;
pub mod writer;

pub type RawData = Vec<u8>;

// pub type RawData = Arc<[u8]>;

// TODO LIST:
// Copy the unit tests below and paste them at the bottom of your chunk.rs file.
// Write a Chunk struct with your implementation of PNG chunks.
// Implement TryFrom<&[u8]> for your Chunk.
// Implement Display for your Chunk.
// Required methods:
//      - fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk
//      - fn length(&self) -> u32
//      - fn chunk_type(&self) -> &ChunkType
//      - fn data(&self) -> &[u8]
//      - fn crc(&self) -> u32
//      - fn data_as_string(&self) -> Result<String>
//      - fn as_bytes(&self) -> Vec<u8>
// Pass all of the unit tests.

use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Debug)]
pub struct Chunk {
    data: Vec<u8>
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err("Invalid Chunk");
        }
        let data_length = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);
        let chunk_type = [value[4], value[5], value[6], value[7]];
        let message_bytes = &value[8..value.len()-4];
        // from_be_bytes: Convert a slice of bytes into a u32 in big-endian order.
        let crc = u32::from_be_bytes([value[value.len()-4], value[value.len()-3], value[value.len()-2], value[value.len()-1]]);
        // crc: Cyclic Redundancy Check
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        let crc_from_data = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&[&chunk_type, message_bytes].concat());
        if chunk_data != value || crc != crc_from_data {
            return Err("Invalid Chunk");
        }
        Ok(Chunk {
            data: value.to_vec() 
        })
    }     
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Chunk: {}", self.data.len())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let data_length: u32 = data.len() as u32;
        let crc32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc = crc32.checksum(&[&chunk_type.bytes(), data.as_slice()].concat());
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.bytes().iter())
            .chain(data.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        Chunk {
            data: chunk_data
        }
    }

    pub fn length(&self) -> u32 {
        u32::from_be_bytes([self.data[0], self.data[1], self.data[2], self.data[3]])
    }

    pub fn chunk_type(&self) -> ChunkType {
        let chunk_type = &self.data[4..8];
        ChunkType::try_from([chunk_type[0], chunk_type[1], chunk_type[2], chunk_type[3]]).unwrap()
    }

    pub fn data(&self) -> &[u8] {
        &self.data[8..self.data.len()-4]
    }

    fn crc(&self) -> u32 {
        u32::from_be_bytes([self.data[self.data.len()-4], self.data[self.data.len()-3], self.data[self.data.len()-2], self.data[self.data.len()-1]])
    }

    pub fn data_as_string(&self) -> Result<String, std::str::Utf8Error> {
        std::str::from_utf8(self.data()).map(|s| s.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
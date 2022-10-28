use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct ChunkType {
    data: String,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

   fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let string_result = std::str::from_utf8(&value);
        match string_result {
            Ok(result) => Ok(ChunkType {
                data: result.to_string(),
            }),
            Err(_) => Err("Error converting from provided bytes"),
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("string is invalid. Not 4 characters");
        }
        if s.chars().all(char::is_alphabetic) {
            return Ok(ChunkType {
                data: s.to_string(),
            });
        } else {
            return Err("Non alphabetic string");
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for ChunkType {
    fn assert_receiver_is_total_eq(&self) {}
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        let mut array = [0u8; 4];
        for (&x, p) in self.data.as_bytes().iter().zip(array.iter_mut()) {
            *p = x;
        }
        array
    }

    pub fn is_valid(&self) -> bool {
        let is_alphabetic = self.data.chars().all(char::is_alphabetic);
        let bytes = self.data.as_bytes();
        let is_reserved = u8::is_ascii_uppercase(bytes.get(2).unwrap());

        is_alphabetic && is_reserved
    }

    pub fn is_critical(&self) -> bool {
        let bytes = self.data.as_bytes();

        u8::is_ascii_uppercase(bytes.get(0).unwrap())
    }

    pub fn is_public(&self) -> bool {
        let bytes = self.data.as_bytes();

        u8::is_ascii_uppercase(bytes.get(1).unwrap())
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let bytes = self.data.as_bytes();

        u8::is_ascii_uppercase(bytes.get(2).unwrap())
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let bytes = self.data.as_bytes();

        u8::is_ascii_lowercase(bytes.get(3).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

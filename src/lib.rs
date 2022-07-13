use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Debug, PartialEq, Deserialize)]
struct TestMixed {
    pub a: i8,
    pub b: u8,
    pub c: i16,
    pub d: u16,
    pub e: i32,
    pub f: u32,
    pub g: i64,
    pub h: u64,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
struct TestAllU8s {
    pub a: u8,
    pub b: u8,
    pub c: u8
}

#[cfg(test)]
mod tests {
    use crate::{TestMixed, TestAllU8s};
    #[test]
    fn validate_mixed_deserializes() {
        let original = TestMixed {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
            e: 1,
            f: 1,
            g: 1,
            h: 1,
        };
        let serialized = bincode::serialize(&original).unwrap();
        println!("Result {:?}", serialized);
        let deserialized_directly: TestMixed = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized_directly, original);
        let mut bincode_deserializer = bincode::Deserializer::with_reader(&serialized[..], bincode::DefaultOptions::new());
        let wrapped_deserialize: TestMixed = serde_path_to_error::deserialize(&mut bincode_deserializer).unwrap();
        assert_eq!(original, wrapped_deserialize);
    }
    #[test]
    fn validate_u8_deserializes() {
        let original = TestAllU8s {
            a: 1,
            b: 1,
            c: 1,
        };
        let serialized = bincode::serialize(&original).unwrap();
        println!("Result {:?}", serialized);
        let deserialized_directly: TestAllU8s= bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized_directly, original);
        let mut bincode_deserializer = bincode::Deserializer::with_reader(&serialized[..], bincode::DefaultOptions::new());
        let wrapped_deserialize: TestAllU8s= serde_path_to_error::deserialize(&mut bincode_deserializer).unwrap();
        assert_eq!(original, wrapped_deserialize);
    }
}

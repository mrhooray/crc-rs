extern crate crc;

mod hasher {
    use crc::{crc16, crc32, crc64};
    use std::hash::{Hash, Hasher};

    #[derive(Hash)]
    struct Person(&'static str, u8);

    #[test]
    fn checksum_hash_crc16() {
        let person = Person("John Smith", 34);
        let mut hasher = crc16::Digest::new(crc16::X25);
        person.hash(&mut hasher);
        assert_eq!(27_228u64, hasher.finish());
    }

    #[test]
    fn checksum_hash_crc32() {
        let person = Person("John Smith", 34);
        let mut hasher = crc32::Digest::new(crc32::IEEE);
        person.hash(&mut hasher);
        assert_eq!(467_823_795u64, hasher.finish());
    }

    #[test]
    fn checksum_hash_crc64() {
        let person = Person("John Smith", 34);
        let mut hasher = crc64::Digest::new(crc64::ECMA);
        person.hash(&mut hasher);
        assert_eq!(3_567_258_626_315_136_489u64, hasher.finish());
    }

}

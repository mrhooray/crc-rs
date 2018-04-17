extern crate crc;

mod hasher {
    use crc::{crc32, crc64};
    use std::hash::{Hash, Hasher};

    #[derive(Hash)]
    struct Person(&'static str, u8);

    #[test]
    fn checksum_hashcrc32() {
        let person = Person("John Smith", 34);
        let mut hasher = crc32::Digest::new_with_initial_and_final(crc32::IEEE, 0xFFFFFFFF, true, 0xFFFFFFFF);
        person.hash(&mut hasher);
        assert_eq!(467823795u64, hasher.finish()); //0x1BE26CB3  //0x6037AADC
    }

    #[test]
    fn checksum_hashcrc64() {
        let person = Person("John Smith", 34);
        let mut hasher = crc64::Digest::new(crc64::ECMA);
        person.hash(&mut hasher);
        assert_eq!(3567258626315136489u64, hasher.finish());
    }

}

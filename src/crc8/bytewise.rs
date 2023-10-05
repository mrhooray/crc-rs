use crate::crc8::{finalize, init, update_bytewise};
use crate::table::crc8_table;
use crate::{Algorithm, Bytewise, Crc, Digest};
use core::hash::{BuildHasher, Hasher};

impl Crc<Bytewise<u8>> {
    pub const fn new(algorithm: &'static Algorithm<u8>) -> Self {
        let table = crc8_table(algorithm.width, algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u8 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u8, bytes: &[u8]) -> u8 {
        update_bytewise(crc, &self.table, bytes)
    }

    pub const fn digest(&self) -> Digest<Bytewise<u8>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u8) -> Digest<Bytewise<u8>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, Bytewise<u8>> {
    const fn new(crc: &'a Crc<Bytewise<u8>>, value: u8) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u8 {
        finalize(self.crc.algorithm, self.value)
    }
}

impl<'a> Hasher for Digest<'a, Bytewise<u8>> {
    fn finish(&self) -> u64 {
        self.clone().finalize() as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
}

impl<'a> BuildHasher for &'a Crc<Bytewise<u8>> {
    type Hasher = Digest<'a, Bytewise<u8>>;

    fn build_hasher(&self) -> Self::Hasher {
        self.digest()
    }
}

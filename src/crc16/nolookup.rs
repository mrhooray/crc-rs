use crate::crc16::{finalize, init, update_nolookup};
use crate::{Algorithm, Crc, Digest, NoTable};
use core::hash::{BuildHasher, Hasher};

impl Crc<NoTable<u16>> {
    pub const fn new(algorithm: &'static Algorithm<u16>) -> Self {
        Self {
            algorithm,
            table: (),
        }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u16 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u16, bytes: &[u8]) -> u16 {
        update_nolookup(crc, self.algorithm, bytes)
    }

    pub const fn digest(&self) -> Digest<NoTable<u16>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u16) -> Digest<NoTable<u16>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, NoTable<u16>> {
    const fn new(crc: &'a Crc<NoTable<u16>>, value: u16) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u16 {
        finalize(self.crc.algorithm, self.value)
    }
}

impl<'a> Hasher for Digest<'a, NoTable<u16>> {
    fn finish(&self) -> u64 {
        self.clone().finalize() as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
}

impl<'a> BuildHasher for &'a Crc<NoTable<u16>> {
    type Hasher = Digest<'a, NoTable<u16>>;

    fn build_hasher(&self) -> Self::Hasher {
        self.digest()
    }
}

pub fn make_table_crc16(poly: u16) -> [u16; 256] {
    let mut table = [0u16; 256];
    for i in 0..256 {
        let mut value = i as u16;
        for _ in 0..8 {
            value = if (value & 1) == 1 {
                (value >> 1) ^ poly
            } else {
                value >> 1
            }
        }
        table[i] = value;
    }
    table
}

pub fn make_table_crc32(poly: u32, rfl: bool) -> [u32; 256] {
    let mut table = [0u32; 256];
    let mask: u32 = 0xFFFFFFFF;
    let hash_size = 32;

    if rfl == true{
        for i in 0..256 {
            let mut value = i as u32;
            for _ in 0..8 {
                value = if (value & 1) == 1 {
                    (value >> 1) ^ poly
                } else {
                    value >> 1
                }
            }
            table[i] = value;
        }
    }
    else{
        println!("FALSE! {}", "!");

        let top_bit = (1u32) << (hash_size - 1); //31 is 32bit - 1

        for i in 0..256 {
            
            //Shift the cuttent table value "i" to the top byte in the long
            let mut value: u32 = (i as u32) << (hash_size - 8);   //24=32 bit - 8
            
            //Step through all the bits in the byte
            for _ in 0..8 {
                if (value & top_bit) != 0 {
                    value = (value << 1) ^ poly
                } else {
                    value <<= 1
                }
            }
            table[i] = value & mask;
        }
    }
    table
}

pub fn make_table_crc64(poly: u64) -> [u64; 256] {
    let mut table = [0u64; 256];
    for i in 0..256 {
        let mut value = i as u64;
        for _ in 0..8 {
            value = if (value & 1) == 1 {
                (value >> 1) ^ poly
            } else {
                value >> 1
            }
        }
        table[i] = value;
    }
    table
}

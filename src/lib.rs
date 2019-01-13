#![feature(reverse_bits)]
#![feature(const_int_conversion)]

use std::mem::size_of;

pub struct Crc<T> {
    crc: T,
    poly: T,
    offset: usize,
    reflect: bool,
    initial: T,
    final_xor: T,
    lookup_table: [T; 256],
}

macro_rules! crc_impl {
    ($($t:tt)*) => ($(
        impl Crc<$t> {
            pub fn new(poly: $t, width: usize, initial: $t, final_xor: $t, reflect: bool) -> Self {
                let offset = size_of::<$t>() * 8 - width;
                let mut crc = Self {
                    crc: initial,
                    poly: poly << offset,
                    offset,
                    reflect,
                    initial,
                    final_xor,
                    lookup_table: [0; 256],
                };
                crc.make_lookup_table();
                crc
            }

            fn byte_crc(&self, byte: u8) -> $t {
                const MASK: $t = (1 as $t).reverse_bits();
                const OFFSET: usize = size_of::<$t>() * 8 - 8;

                let mut crc = (byte as $t) << OFFSET;
                for _ in 0..8 {
                    if crc & MASK == MASK {
                        crc <<= 1;
                        crc = crc ^ self.poly;
                    } else {
                        crc <<= 1;
                    }
                }
                crc
            }

            fn make_lookup_table(&mut self) {
                if self.reflect {
                    for i in 0..256 {
                        self.lookup_table[i] = self.byte_crc((i as u8).reverse_bits()).reverse_bits();
                    }
                } else {
                    for i in 0..256 {
                        self.lookup_table[i] = self.byte_crc(i as u8);
                    }
                }
            }

            pub fn update(&mut self, data: &[u8]) -> $t {
                macro_rules! update {
                    (u8) => {
                        if !self.reflect {
                            self.crc <<= self.offset;
                        }

                        for i in 0..data.len() {
                            self.crc = self.lookup_table[(self.crc ^ data[i]) as usize];
                        }
                    };
                    ($_:ty) => {
                        if self.reflect {
                            for i in 0..data.len() {
                                self.crc = self.crc >> 8 ^ self.lookup_table[(self.crc.to_le_bytes()[0] ^ data[i]) as usize];
                            }
                        } else {
                            self.crc <<= self.offset;
                            for i in 0..data.len() {
                                self.crc = self.crc << 8 ^ self.lookup_table[(self.crc.to_be_bytes()[0] ^ data[i]) as usize];
                            }
                        }
                    };
                }
                update!($t);

                self.crc()
            }

            pub fn crc(&self) -> $t {
                if self.reflect {
                    self.crc ^ self.final_xor
                } else {
                    self.crc >> self.offset ^ self.final_xor
                }
            }

            pub fn init(&mut self) {
                self.crc = self.initial;
            }
        }
    )*)
}

crc_impl!(u8 u16 u32 u64);

#[cfg(test)]
mod tests;

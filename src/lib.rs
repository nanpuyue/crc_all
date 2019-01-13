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
                    poly: if reflect { (poly << offset).reverse_bits() } else { poly << offset },
                    offset,
                    reflect,
                    initial,
                    final_xor,
                    lookup_table: [0; 256],
                };
                crc.make_lookup_table();
                crc
            }

            fn make_lookup_table(&mut self) {
                if self.reflect {
                    for (i, v) in self.lookup_table.iter_mut().enumerate() {
                        *v = i as $t;
                        for _ in 0..8 {
                            if *v & 1 == 1 {
                                *v >>= 1;
                                *v = *v ^ self.poly;
                            } else {
                                *v >>= 1;
                            }
                        }
                    }
                } else {
                    const MASK: $t = (1 as $t).reverse_bits();
                    const OFFSET: usize = size_of::<$t>() * 8 - 8;

                    for (i, v) in self.lookup_table.iter_mut().enumerate() {
                        *v = (i as $t) << OFFSET;
                        for _ in 0..8 {
                            if *v & MASK == MASK {
                                *v <<= 1;
                                *v = *v ^ self.poly;
                            } else {
                                *v <<= 1;
                            }
                        }
                    }
                }
            }

            pub fn update(&mut self, data: &[u8]) -> $t {
                macro_rules! update {
                    (u8) => {
                        if !self.reflect {
                            self.crc <<= self.offset;
                        }

                        for b in data {
                            self.crc = self.lookup_table[(self.crc ^ b) as usize];
                        }
                    };
                    ($_:ty) => {
                        if self.reflect {
                            for b in data {
                                self.crc = self.crc >> 8 ^ self.lookup_table[(self.crc.to_le_bytes()[0] ^ b) as usize];
                            }
                        } else {
                            self.crc <<= self.offset;
                            for b in data {
                                self.crc = self.crc << 8 ^ self.lookup_table[(self.crc.to_be_bytes()[0] ^ b) as usize];
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

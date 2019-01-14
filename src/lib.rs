#![feature(reverse_bits)]
#![feature(const_int_conversion)]
#![allow(mutable_transmutes)]

use std::mem::{size_of, transmute};

pub struct Crc<T> {
    crc: T,
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
                Self {
                    crc: initial,
                    offset: size_of::<$t>() * 8 - width,
                    reflect,
                    initial,
                    final_xor,
                    lookup_table: Self::make_lookup_table(poly, width, reflect),
                }
            }

            pub fn make_lookup_table(poly: $t, width: usize, reflect: bool) -> [$t; 256] {
                let offset = size_of::<$t>() * 8 - width;
                let poly = if reflect { (poly << offset).reverse_bits() } else { poly << offset };

                let mut lookup_table = [0 as $t; 256];
                if reflect {
                    for (i, v) in lookup_table.iter_mut().enumerate() {
                        *v = i as $t;
                        for _ in 0..8 {
                            if *v & 1 == 1 {
                                *v >>= 1;
                                *v = *v ^ poly;
                            } else {
                                *v >>= 1;
                            }
                        }
                    }
                } else {
                    const MASK: $t = (1 as $t).reverse_bits();
                    const OFFSET: usize = size_of::<$t>() * 8 - 8;

                    for (i, v) in lookup_table.iter_mut().enumerate() {
                        *v = (i as $t) << OFFSET;
                        for _ in 0..8 {
                            if *v & MASK == MASK {
                                *v <<= 1;
                                *v = *v ^ poly;
                            } else {
                                *v <<= 1;
                            }
                        }
                    }
                }
                lookup_table
            }

            pub fn update_crc(&self, crc: &mut $t, data: &[u8]) -> $t {
                macro_rules! update {
                    (u8) => {
                        if !self.reflect {
                            *crc <<= self.offset;
                        }

                        for b in data {
                            *crc = self.lookup_table[(*crc ^ b) as usize];
                        }
                    };
                    ($_:ty) => {
                        if self.reflect {
                            for b in data {
                                *crc = *crc >> 8 ^ self.lookup_table[(crc.to_le_bytes()[0] ^ b) as usize];
                            }
                        } else {
                            *crc <<= self.offset;
                            for b in data {
                                *crc = *crc << 8 ^ self.lookup_table[(crc.to_be_bytes()[0] ^ b) as usize];
                            }
                        }
                    };
                }
                update!($t);

                self.final_crc(crc)
            }

            pub fn update(&mut self, data: &[u8]) -> $t {
                unsafe {
                    self.update_crc(transmute::<_, &mut $t>(&self.crc), data)
                }
            }

            pub fn final_crc(&self, crc: &$t) -> $t {
                if self.reflect {
                    crc ^ self.final_xor
                } else {
                    crc >> self.offset ^ self.final_xor
                }
            }

            pub fn r#final(&self) -> $t {
                self.final_crc(&self.crc)
            }

            pub fn init_crc(&self, crc: &mut $t) {
                *crc = self.initial;
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

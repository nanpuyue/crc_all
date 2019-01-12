#![feature(reverse_bits)]

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
    ($($t:ty)*) => ($(
        impl Crc<$t> {
            pub fn new(poly: $t, width: usize, initial: $t, final_xor: $t, reflect: bool) -> Self {
                let offset = size_of::<$t>() * 8 - width;
                let mut crc = Self {
                    crc: initial,
                    poly,
                    offset,
                    reflect,
                    initial,
                    final_xor,
                    lookup_table: [0 as $t; 256],
                };
                crc.make_lookup_table();
                crc
            }

            fn byte_crc(&self, byte: u8) -> $t {
                let mut crc = byte;
                let poly = self.poly << self.offset;

                for _ in 0..8 {
                    if crc & 128 == 128 {
                        crc <<= 1;
                        crc = crc ^ poly;
                    } else {
                        crc <<= 1;
                    }
                }
                crc
            }

            fn make_lookup_table(&mut self) {
                if self.reflect {
                    for i in 0..256usize {
                        self.lookup_table[i] = self.byte_crc((i as u8).reverse_bits()).reverse_bits();
                    }
                } else {
                    for i in 0..256usize {
                        self.lookup_table[i] = self.byte_crc(i as u8);
                    }
                }
            }

            pub fn update(&mut self, data: &[u8]) -> $t {
                if !self.reflect {
                    self.crc <<= self.offset;
                }

                for i in 0..data.len() {
                    self.crc = self.lookup_table[(self.crc ^ data[i]) as usize];
                }

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

crc_impl!(u8);

#[cfg(test)]
mod tests;

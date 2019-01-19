#![no_std]

#![feature(reverse_bits)]
#![allow(mutable_transmutes)]

use core::mem::{size_of, transmute};

pub struct Crc<T> {
    crc: T,
    offset: usize,
    reflect: bool,
    init: T,
    xorout: T,
    table: [T; 256],
}

macro_rules! crc_impl {
    ($($t:tt)*) => ($(
        impl Crc<$t> {
            pub fn new(poly: $t, width: usize, init: $t, xorout: $t, reflect: bool) -> Self {
                let offset = size_of::<$t>() * 8 - width;
                Self {
                    crc: if reflect { init.reverse_bits() >> offset } else { init },
                    offset,
                    reflect,
                    init,
                    xorout,
                    table: Self::make_table(poly, width, reflect),
                }
            }

            pub fn make_table(poly: $t, width: usize, reflect: bool) -> [$t; 256] {
                let offset = size_of::<$t>() * 8 - width;
                let poly = if reflect { (poly << offset).reverse_bits() } else { poly << offset };

                let mut table = [0 as $t; 256];
                if reflect {
                    for (i, v) in table.iter_mut().enumerate() {
                        *v = i as $t;
                        for _ in 0..8 {
                            if v.trailing_zeros() == 0 {
                                *v = *v >> 1 ^ poly;
                            } else {
                                *v >>= 1;
                            }
                        }
                    }
                } else {
                    for (i, v) in table.iter_mut().enumerate() {
                        *v = (i as $t) << size_of::<$t>() * 8 - 8;
                        for _ in 0..8 {
                            if v.leading_zeros() == 0 {
                                *v = *v << 1 ^ poly;
                            } else {
                                *v <<= 1;
                            }
                        }
                    }
                }
                table
            }

            pub fn update_crc(&self, crc: &mut $t, data: &[u8]) -> $t {
                macro_rules! crc_update {
                    (u8) => {
                        if !self.reflect {
                            *crc <<= self.offset;
                        }

                        for b in data {
                            *crc = self.table[(*crc ^ b) as usize];
                        }
                    };
                    ($_:ty) => {
                        if self.reflect {
                            for b in data {
                                *crc = *crc >> 8 ^ self.table[(crc.to_le_bytes()[0] ^ b) as usize];
                            }
                        } else {
                            *crc <<= self.offset;
                            for b in data {
                                *crc = *crc << 8 ^ self.table[(crc.to_be_bytes()[0] ^ b) as usize];
                            }
                        }
                    };
                }
                crc_update!($t);

                self.final_crc(crc)
            }

            pub fn update(&mut self, data: &[u8]) -> $t {
                unsafe {
                    self.update_crc(transmute::<_, &mut $t>(&self.crc), data)
                }
            }

            pub fn final_crc(&self, crc: &$t) -> $t {
                if self.reflect {
                    crc ^ self.xorout
                } else {
                    crc >> self.offset ^ self.xorout
                }
            }

            pub fn r#final(&self) -> $t {
                self.final_crc(&self.crc)
            }

            pub fn init_crc(&self, crc: &mut $t) {
                *crc = self.init;
            }

            pub fn init(&mut self) {
                self.crc = self.init;
            }
        }
    )*)
}

crc_impl!(u8 u16 u32 u64 u128);

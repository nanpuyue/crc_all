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
mod tests {
    use super::*;

    #[test]
    fn verify_crc() {
        let data0 = b"c4ca4238a0b923820dcc509a6f75849b".as_ref();
        let data1 = b"c81e728d9d4c2f636f067f89cc14862c".as_ref();

        let mut crc5usb = Crc::new(0x05, 5, 0x1f, 0x1f, true);
        assert_eq!(crc5usb.update(data0), 0x17);
        crc5usb.init();
        assert_eq!(crc5usb.update(data1), 0x1c);

        let mut crc5x = Crc::new(0x05, 5, 0x1f, 0x1f, false);
        assert_eq!(crc5x.update(data0), 0x06);
        crc5x.init();
        assert_eq!(crc5x.update(data1), 0x02);

        let mut crc8 = Crc::new(0x07, 8, 0, 0, false);
        assert_eq!(crc8.update(data0), 0x47);
        crc8.init();
        assert_eq!(crc8.update(data1), 0x45);

        let mut crc8rohc = Crc::new(0x07, 8, 0xff, 0, true);
        assert_eq!(crc8rohc.update(data0), 0x12);
        crc8rohc.init();
        assert_eq!(crc8rohc.update(data1), 0xcb);
    }
}

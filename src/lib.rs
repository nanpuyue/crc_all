//! # A Pure Rust Implementation of Generic CRC Algorithm
//!
//! [![Crates.io](https://img.shields.io/crates/v/crc_all?color=green)](https://crates.io/crates/crc_all)
//!
//! ## Supported Algorithms
//!
//! `CRC-3/GSM`,`CRC-3/ROHC`,`CRC-4/G-704`,`CRC-4/INTERLAKEN`,`CRC-5/EPC-C1G2`,`CRC-5/G-704`,`CRC-5/USB`,`CRC-6/CDMA2000-A`,
//! `CRC-6/CDMA2000-B`,`CRC-6/DARC`,`CRC-6/G-704`,`CRC-6/GSM`,`CRC-7/MMC`,`CRC-7/ROHC`,`CRC-7/UMTS`,`CRC-8/AUTOSAR`,
//! `CRC-8/BLUETOOTH`,`CRC-8/CDMA2000`,`CRC-8/DARC`,`CRC-8/DVB-S2`,`CRC-8/GSM-A`,`CRC-8/GSM-B`,`CRC-8/I-432-1`,
//! `CRC-8/I-CODE`,`CRC-8/LTE`,`CRC-8/MAXIM-DOW`,`CRC-8/NRSC-5`,`CRC-8/OPENSAFETY`,`CRC-8/ROHC`,`CRC-8/SAE-J1850`,
//! `CRC-8/SMBUS`,`CRC-8/TECH-3250`,`CRC-8/WCDMA`,`CRC-10/ATM`,`CRC-10/CDMA2000`,`CRC-10/GSM`,`CRC-11/FLEXRAY`,
//! `CRC-11/UMTS`,`CRC-12/CDMA2000`,`CRC-12/DECT`,`CRC-12/GSM`,`CRC-12/UMTS`,`CRC-13/BBC`,`CRC-14/DARC`,`CRC-14/GSM`,
//! `CRC-15/CAN`,`CRC-15/MPT1327`,`CRC-16/ARC`,`CRC-16/CDMA2000`,`CRC-16/CMS`,`CRC-16/DDS-110`,`CRC-16/DECT-R`,
//! `CRC-16/DECT-X`,`CRC-16/DNP`,`CRC-16/EN-13757`,`CRC-16/GENIBUS`,`CRC-16/GSM`,`CRC-16/IBM-3740`,`CRC-16/IBM-SDLC`,
//! `CRC-16/ISO-IEC-14443-3-A`,`CRC-16/KERMIT`,`CRC-16/LJ1200`,`CRC-16/MAXIM-DOW`,`CRC-16/MCRF4XX`,`CRC-16/MODBUS`,
//! `CRC-16/NRSC-5`,`CRC-16/OPENSAFETY-A`,`CRC-16/OPENSAFETY-B`,`CRC-16/PROFIBUS`,`CRC-16/RIELLO`,`CRC-16/SPI-FUJITSU`,
//! `CRC-16/T10-DIF`,`CRC-16/TELEDISK`,`CRC-16/TMS37157`,`CRC-16/UMTS`,`CRC-16/USB`,`CRC-16/XMODEM`,`CRC-17/CAN-FD`,
//! `CRC-21/CAN-FD`,`CRC-24/BLE`,`CRC-24/FLEXRAY-A`,`CRC-24/FLEXRAY-B`,`CRC-24/INTERLAKEN`,`CRC-24/LTE-A`,`CRC-24/LTE-B`,
//! `CRC-24/OPENPGP`,`CRC-24/OS-9`,`CRC-30/CDMA`,`CRC-31/PHILIPS`,`CRC-32/AIXM`,`CRC-32/AUTOSAR`,`CRC-32/BASE91-D`,
//! `CRC-32/BZIP2`,`CRC-32/CKSUM`,`CRC-32/ISCSI`,`CRC-32/ISO-HDLC`,`CRC-32/JAMCRC`,`CRC-32/MPEG-2`,`CRC-32/XFER`,
//! `CRC-40/GSM`,`CRC-64/ECMA-182`,`CRC-64/GO-ISO`,`CRC-64/WE`,`CRC-64/XZ`,`CRC-82/DARC`
//!
//! See `CRC.txt`.
//!
//! **Note:** `CRC-12/UMTS` need special operation, see `tests/tests.rs`.
//!
//! ## Usage
//!
//! Add `crc_all` to `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! crc_all = "0.2.2"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use crc_all::CrcAlgo;
//!
//! fn crc11_umts(data: &[u8]) -> u16 {
//!     const CRC11_UMTS: CrcAlgo<u16> = CrcAlgo::<u16>::new(0x307, 11, 0, 0, false);
//!
//!     let crc = &mut 0u16;
//!     CRC11_UMTS.init_crc(crc);
//!     CRC11_UMTS.update_crc(crc, data)
//! }
//!
//! assert_eq!(crc11_umts(b"123456789".as_ref()), 0x061);
//! ```
//!
//! ```rust
//! use crc_all::Crc;
//!
//! let data = b"123456789".as_ref();
//! let mut crc5_usb = Crc::<u8>::new(0x05, 5, 0x1f, 0x1f, true);
//!
//! assert_eq!(crc5_usb.update(data), 0x19);
//! assert_eq!(crc5_usb.update(data), 0x03);
//!
//! crc5_usb.init();
//! assert_eq!(crc5_usb.update(data), 0x19);
//! ```

#![no_std]

use core::mem::size_of;

pub struct CrcAlgo<T> {
    poly: T,
    offset: usize,
    init: T,
    xorout: T,
    reflect: bool,
    table: [T; 256],
}

pub struct Crc<T> {
    crc: T,
    algo: CrcAlgo<T>,
}

macro_rules! crc_impl {
    ($($t:tt)*) => ($(
        impl CrcAlgo<$t> {
            const fn make_table(poly: $t, reflect: bool)-> [$t; 256] {
                let mut table = [0 as $t; 256];

                if reflect {
                    const fn table_value(index: usize, poly: $t) -> $t {
                        let mut v = index as $t;
                        let mut i = 0;
                        while i < 8 {
                            if v.trailing_zeros() == 0 {
                                v = v >> 1 ^ poly;
                            } else {
                                v >>= 1;
                            }
                            i += 1;
                        }
                        v
                    }

                    let mut i = 0;
                    while i < 256 {
                        table[i] = table_value(i, poly);
                        i += 1;
                    }
                } else {
                    const fn table_value(index: usize, poly: $t) -> $t {
                        let mut v = (index as $t) << size_of::<$t>() * 8 - 8;
                        let mut i = 0;
                        while i < 8 {
                            if v.leading_zeros() == 0 {
                                v = v << 1 ^ poly;
                            } else {
                                v <<= 1;
                            }
                            i += 1;
                        }
                        v
                    }

                    let mut i = 0;
                    while i < 256 {
                        table[i] = table_value(i, poly);
                        i += 1;
                    }
                }

                table
            }

            pub const fn new(poly: $t, width: usize, init: $t, xorout: $t, reflect: bool) -> Self {
                let offset = size_of::<$t>() * 8 - width;
                let poly = if reflect { (poly << offset).reverse_bits() } else { poly << offset };
                let init = if reflect { init.reverse_bits() >> offset } else { init };
                Self {
                    poly,
                    offset,
                    init,
                    xorout,
                    reflect,
                    table: Self::make_table(poly, reflect),
                }
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

                self.finish_crc(crc)
            }

            /// The bits `0b01010000` with offset `3` means `0b01010`.
            ///
            /// # Panics
            ///
            /// Panics if `self.reflect` is `true` or `offset >= 8`.
            pub fn update_bits_crc(&self, crc: &mut $t, bits: u8, offset: usize) -> $t {
                assert!(!self.reflect);
                assert!(offset < 8);

                *crc ^= ((bits & 0xff << offset) as $t) << ((size_of::<$t>() - 1) * 8);
                for _ in offset..8 {
                    if crc.leading_zeros() == 0 {
                        *crc = *crc << 1 ^ self.poly;
                    } else {
                        *crc <<= 1;
                    }
                }

                self.finish_crc(crc)
            }

            pub fn finish_crc(&self, crc: &$t) -> $t {
                if self.reflect {
                    crc ^ self.xorout
                } else {
                    crc >> self.offset ^ self.xorout
                }
            }

            pub fn init_crc(&self, crc: &mut $t) {
                *crc = self.init;
            }
        }

        impl Crc<$t> {
            pub const fn new(poly: $t, width: usize, init: $t, xorout: $t, reflect: bool) -> Self {
                let algo: CrcAlgo<$t> = CrcAlgo::<$t>::new(poly, width, init, xorout, reflect);
                Self {
                    crc: algo.init,
                    algo
                }
            }

            pub fn update(&mut self, data: &[u8]) -> $t {
                self.algo.update_crc(&mut self.crc, data)
            }

            /// See `CrcAlgo::update_bits_crc()`.
            pub fn update_bits(&mut self, bits: u8, offset: usize) -> $t {
                self.algo.update_bits_crc(&mut self.crc, bits, offset)
            }

            pub fn finish(&self) -> $t {
                self.algo.finish_crc(&self.crc)
            }

            pub fn init(&mut self) {
                self.crc = self.algo.init;
            }
        }
    )*)
}

crc_impl!(u8 u16 u32 u64 u128);

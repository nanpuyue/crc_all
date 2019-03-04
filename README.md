# A Pure Rust Implementation of Generic CRC Algorithm

## Supported Algorithms

`CRC-3/GSM`,`CRC-3/ROHC`,`CRC-4/G-704`,`CRC-4/INTERLAKEN`,`CRC-5/EPC-C1G2`,`CRC-5/G-704`,`CRC-5/USB`,`CRC-6/CDMA2000-A`,
`CRC-6/CDMA2000-B`,`CRC-6/DARC`,`CRC-6/G-704`,`CRC-6/GSM`,`CRC-7/MMC`,`CRC-7/ROHC`,`CRC-7/UMTS`,`CRC-8/AUTOSAR`,
`CRC-8/BLUETOOTH`,`CRC-8/CDMA2000`,`CRC-8/DARC`,`CRC-8/DVB-S2`,`CRC-8/GSM-A`,`CRC-8/GSM-B`,`CRC-8/I-432-1`,
`CRC-8/I-CODE`,`CRC-8/LTE`,`CRC-8/MAXIM-DOW`,`CRC-8/NRSC-5`,`CRC-8/OPENSAFETY`,`CRC-8/ROHC`,`CRC-8/SAE-J1850`,
`CRC-8/SMBUS`,`CRC-8/TECH-3250`,`CRC-8/WCDMA`,`CRC-10/ATM`,`CRC-10/CDMA2000`,`CRC-10/GSM`,`CRC-11/FLEXRAY`,
`CRC-11/UMTS`,`CRC-12/CDMA2000`,`CRC-12/DECT`,`CRC-12/GSM`,`CRC-12/UMTS`,`CRC-13/BBC`,`CRC-14/DARC`,`CRC-14/GSM`,
`CRC-15/CAN`,`CRC-15/MPT1327`,`CRC-16/ARC`,`CRC-16/CDMA2000`,`CRC-16/CMS`,`CRC-16/DDS-110`,`CRC-16/DECT-R`,
`CRC-16/DECT-X`,`CRC-16/DNP`,`CRC-16/EN-13757`,`CRC-16/GENIBUS`,`CRC-16/GSM`,`CRC-16/IBM-3740`,`CRC-16/IBM-SDLC`,
`CRC-16/ISO-IEC-14443-3-A`,`CRC-16/KERMIT`,`CRC-16/LJ1200`,`CRC-16/MAXIM-DOW`,`CRC-16/MCRF4XX`,`CRC-16/MODBUS`,
`CRC-16/NRSC-5`,`CRC-16/OPENSAFETY-A`,`CRC-16/OPENSAFETY-B`,`CRC-16/PROFIBUS`,`CRC-16/RIELLO`,`CRC-16/SPI-FUJITSU`,
`CRC-16/T10-DIF`,`CRC-16/TELEDISK`,`CRC-16/TMS37157`,`CRC-16/UMTS`,`CRC-16/USB`,`CRC-16/XMODEM`,`CRC-17/CAN-FD`,
`CRC-21/CAN-FD`,`CRC-24/BLE`,`CRC-24/FLEXRAY-A`,`CRC-24/FLEXRAY-B`,`CRC-24/INTERLAKEN`,`CRC-24/LTE-A`,`CRC-24/LTE-B`,
`CRC-24/OPENPGP`,`CRC-24/OS-9`,`CRC-30/CDMA`,`CRC-31/PHILIPS`,`CRC-32/AIXM`,`CRC-32/AUTOSAR`,`CRC-32/BASE91-D`,
`CRC-32/BZIP2`,`CRC-32/CKSUM`,`CRC-32/ISCSI`,`CRC-32/ISO-HDLC`,`CRC-32/JAMCRC`,`CRC-32/MPEG-2`,`CRC-32/XFER`,
`CRC-40/GSM`,`CRC-64/ECMA-182`,`CRC-64/GO-ISO`,`CRC-64/WE`,`CRC-64/XZ`,`CRC-82/DARC`

See `CRC.txt`.

**Note:** `CRC-12/UMTS` need special operation, see `tests/tests.rs`.

## Usage

Add crc_all to Cargo.toml

```toml
[dependencies]
crc_all = { git = "https://github.com/nanpuyue/crc" }
```

## Example

```rust
use crc_all::Crc;

fn main() {
    let data = b"123456789".as_ref();
    let mut crc5_usb = Crc::<u8>::new(0x05, 5, 0x1f, 0x1f, true);
    assert_eq!(crc5_usb.update(data), 0x19);
    assert_eq!(crc5_usb.update(data), 0x03);

    crc5_usb.init();
    assert_eq!(crc5_usb.update(data), 0x19);
}
```

```rust
use crc_all::CrcAlgo;
use lazy_static::lazy_static;

fn crc11_umts(data: &[u8]) -> u16 {
    lazy_static! {
        static ref CRC11_UMTS: CrcAlgo<u16> = CrcAlgo::<u16>::new(0x307, 11, 0, 0, false);
    }

    let crc = &mut 0u16;
    CRC11_UMTS.init_crc(crc);
    CRC11_UMTS.update_crc(crc, data)
}

fn main() {
    assert_eq!(crc11_umts(b"123456789".as_ref()), 0x061);
}
```

## License

Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

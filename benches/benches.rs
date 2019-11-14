#![feature(test)]

extern crate test;

use crc_all::Crc;
use test::Bencher;

#[bench]
fn crc08_update_megabytes(b: &mut Bencher) {
    let mut data = Vec::with_capacity(1000000);
    unsafe {
        data.set_len(1000000);
    }

    let mut crc8_smbus = Crc::<u8>::new(0x07, 8, 0, 0, false);

    b.iter(|| {
        crc8_smbus.init();
        crc8_smbus.update(&data);
    });
}

#[bench]
fn crc12_update_megabytes(b: &mut Bencher) {
    let mut data = Vec::with_capacity(1000000);
    unsafe {
        data.set_len(1000000);
    }

    let mut crc12_umts = Crc::<u16>::new(0x80f, 12, 0, 0, false);

    b.iter(|| {
        crc12_umts.init();
        crc12_umts.update(&data);
    });
}

#[bench]
fn crc16_update_megabytes(b: &mut Bencher) {
    let mut data = Vec::with_capacity(1000000);
    unsafe {
        data.set_len(1000000);
    }

    let mut crc16_ibm_sdlc = Crc::<u16>::new(0x1021, 16, 0xffff, 0xffff, true);

    b.iter(|| {
        crc16_ibm_sdlc.init();
        crc16_ibm_sdlc.update(&data);
    });
}

#[bench]
fn crc32_update_megabytes(b: &mut Bencher) {
    let mut data = Vec::with_capacity(1000000);
    unsafe {
        data.set_len(1000000);
    }

    let mut crc32_iso_hdlc = Crc::<u32>::new(0x04c11db7, 32, 0xffffffff, 0xffffffff, true);

    b.iter(|| {
        crc32_iso_hdlc.init();
        crc32_iso_hdlc.update(&data);
    });
}

#[bench]
fn crc64_update_megabytes(b: &mut Bencher) {
    let mut data = Vec::with_capacity(1000000);
    unsafe {
        data.set_len(1000000);
    }

    let mut crc64_ecma182 = Crc::<u64>::new(0x42f0e1eba9ea3693, 64, 0, 0, false);

    b.iter(|| {
        crc64_ecma182.init();
        crc64_ecma182.update(&data);
    });
}

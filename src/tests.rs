use super::*;

#[test]
fn verify_crc() {
    let data0 = b"c4ca4238a0b923820dcc509a6f75849b".as_ref();
    let data1 = b"c81e728d9d4c2f636f067f89cc14862c".as_ref();

    let mut crc5usb = Crc::<u8>::new(0x05, 5, 0x1f, 0x1f, true);
    assert_eq!(crc5usb.update(data0), 0x17);
    crc5usb.init();
    assert_eq!(crc5usb.update(data1), 0x1c);

    let mut crc5x = Crc::<u16>::new(0x05, 5, 0x1f, 0x1f, false);
    assert_eq!(crc5x.update(data0), 0x06);
    crc5x.init();
    assert_eq!(crc5x.update(data1), 0x02);

    let mut crc8 = Crc::<u8>::new(0x07, 8, 0, 0, false);
    assert_eq!(crc8.update(data0), 0x47);
    crc8.init();
    assert_eq!(crc8.update(data1), 0x45);

    let mut crc8rohc = Crc::<u32>::new(0x07, 8, 0xff, 0, true);
    assert_eq!(crc8rohc.update(data0), 0x12);
    assert_eq!(crc8rohc.update(data1), 0x80);

    let mut crc16ibm = Crc::<u16>::new(0x8005, 16, 0, 0, true);
    assert_eq!(crc16ibm.update(data0), 0x7cdc);
    crc16ibm.init();
    assert_eq!(crc16ibm.update(data1), 0x4688);

    let mut crc32 = Crc::<u32>::new(0x04c11db7, 32, 0xffffffff, 0xffffffff, true);
    assert_eq!(crc32.update(data0), 0x2a4c8df2);
    crc32.init();
    assert_eq!(crc32.update(data1), 0x733c52f5);

    let mut crc32mpeg2 = Crc::<u32>::new(0x04c11db7, 32, 0xffffffff, 0, false);
    assert_eq!(crc32mpeg2.update(data0), 0x258e713c);
    assert_eq!(crc32mpeg2.update(data1), 0x3373e173);

    let mut crc32mpeg2x = Crc::<u64>::new(0x04c11db7, 32, 0xffffffff, 0, false);
    assert_eq!(crc32mpeg2x.update(data0), 0x258e713c);
    crc32mpeg2x.init();
    assert_eq!(crc32mpeg2x.update(data1), 0x4bd88512);
}

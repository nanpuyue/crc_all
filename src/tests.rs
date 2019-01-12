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

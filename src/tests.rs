use super::*;

#[test]
fn check_all() {
    let data = b"123456789".as_ref();
    let (mut width, mut poly, mut init, mut reflect, mut xorout, mut check, mut name);

    macro_rules! check {
        ($t:tt) => {
            println!("{: <24}\t0x{}\t0x{}\t0x{}\t{: >5} ...", name, poly, init, xorout, reflect);
            let mut crc = Crc::<$t>::new(
                $t::from_str_radix(poly, 16).unwrap(),
                width,
                $t::from_str_radix(init, 16).unwrap(),
                $t::from_str_radix(xorout, 16).unwrap(),
                reflect
            );
            match name {
                "CRC-12/UMTS" => {
                    assert_eq!(crc.update(data).reverse_bits() >> 4, $t::from_str_radix(check, 16).unwrap());
                },
                _ => {
                    assert_eq!(crc.update(data), $t::from_str_radix(check, 16).unwrap());
                }
            }
        }
    }

    let mut temp: Vec<_>;
    for line in include_str!("../CRC.txt").lines() {
        temp = line.split(|c| c == ' ' || c == '=').collect();
        width = usize::from_str_radix(temp[1], 10).unwrap();
        poly = temp[3].trim_start_matches("0x");
        init = temp[5].trim_start_matches("0x");
        reflect = temp[7] == "true";
        xorout = temp[11].trim_start_matches("0x");
        check = temp[13].trim_start_matches("0x");
        name = temp[17].trim_start_matches('"').trim_end_matches('"');

        let mut n = 8usize;
        while width > n { n *= 2; }
        match n {
            8 => { check!(u8); },
            16 => { check!(u16); },
            32 => { check!(u32); },
            64 => { check!(u64); },
            128 => { check!(u128); },
            _ => ()
        }
    }
}

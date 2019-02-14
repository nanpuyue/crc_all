#![feature(reverse_bits)]

use crc::Crc;

#[test]
fn check_all() {
    let data = b"123456789".as_ref();
    let (mut width, mut poly, mut init, mut reflect, mut xorout, mut check, mut name);

    macro_rules! crc_check {
        ($t:tt) => {{
            println!(
                "{: <24}\t{}\t{}\t{}\t{: >5} ...",
                name, poly, init, xorout, reflect
            );

            let mut crc = Crc::<$t>::new(
                $t::from_str_radix(&poly[2..], 16).unwrap(),
                width,
                $t::from_str_radix(&init[2..], 16).unwrap(),
                $t::from_str_radix(&xorout[2..], 16).unwrap(),
                reflect,
            )
            .update(data);

            if name == "CRC-12/UMTS" {
                crc = crc.reverse_bits() >> 4;
            }

            assert_eq!(crc, $t::from_str_radix(&check[2..], 16).unwrap());
        }};
    }

    let mut params: Vec<_>;
    for line in include_str!("../CRC.txt").lines() {
        params = line
            .split_whitespace()
            .flat_map(|s| s.split("=").skip(1))
            .collect();
        width = usize::from_str_radix(params[0], 10).unwrap();
        poly = params[1];
        init = params[2];
        reflect = params[3] == "true";
        xorout = params[5];
        check = params[6];
        name = params[8].trim_matches('"');

        let mut n = 8usize;
        while width > n {
            n *= 2;
        }

        match n {
            8 => crc_check!(u8),
            16 => crc_check!(u16),
            32 => crc_check!(u32),
            64 => crc_check!(u64),
            128 => crc_check!(u128),
            _ => (),
        }
    }
}

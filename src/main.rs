use aes::*;
use calc_chikey::*;

fn main() {
    let round10_key_raw = [
        0x3f, 0x42, 0x33, 0x7d,
        0x08, 0x39, 0x9e, 0xc2,
        0x8a, 0x89, 0x96, 0xe5,
        0x7c, 0xeb, 0x59, 0x17,
    ];
    let round10_key = GF256::from_u8array(&round10_key_raw).unwrap();

    let round_keys = inv_generate_round_keys(&round10_key);

    for (i, key) in round_keys.iter().enumerate() {
        let p = format!("round{} key", i);
        dump_array16(key, &p);
    }
}

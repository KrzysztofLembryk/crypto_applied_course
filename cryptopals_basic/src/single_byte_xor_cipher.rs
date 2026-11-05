use std::collections::BTreeMap;
use std::sync::LazyLock;

pub static ENG_LETTER_FREQ_TABLE: LazyLock<BTreeMap<char, f64>> =
    LazyLock::new(|| {
        BTreeMap::from([
            ('a', 8.167),
            ('b', 1.492),
            ('c', 2.782),
            ('d', 4.253),
            ('e', 12.702),
            ('f', 2.228),
            ('g', 2.015),
            ('h', 6.094),
            ('i', 6.966),
            ('j', 0.153),
            ('k', 0.772),
            ('l', 4.025),
            ('m', 2.406),
            ('n', 6.749),
            ('o', 7.507),
            ('p', 1.929),
            ('q', 0.095),
            ('r', 5.987),
            ('s', 6.327),
            ('t', 9.056),
            ('u', 2.758),
            ('v', 0.978),
            ('w', 2.361),
            ('x', 0.150),
            ('y', 1.974),
            ('z', 0.074),
        ])
});

use std::str;

pub fn decrypt_byte_xor_cipher(encrypted_hex: &str) {
    let ciphertext = hex::decode(encrypted_hex).unwrap();

    let mut best_score = f64::NEG_INFINITY;
    let mut best_key = 0;
    let mut best_plaintext = Vec::new();

    for key in 0u8..=u8::MAX {
        let plaintext = one_char_xor(&ciphertext, key);
        let score = english_score(&plaintext);

        // The key producing the highest score is selected as the most likely key.
        if score > best_score {
            best_score = score;
            best_key = key;
            best_plaintext = plaintext;
        }
    }

    println!("key: {best_key:#04x}");
    println!("plaintext: {}", String::from_utf8_lossy(&best_plaintext));
}

fn one_char_xor(input: &[u8], key: u8) -> Vec<u8> {
    input.iter().map(|byte| byte ^ key).collect()
}

fn english_score(input: &[u8]) -> f64 {
    input
        .iter()
        .map(|&byte| match byte {
            b'a'..=b'z' => ENG_LETTER_FREQ_TABLE
                .get(&(byte as char))
                .copied()
                .unwrap_or(0.0),

            b'A'..=b'Z' => ENG_LETTER_FREQ_TABLE
                .get(&((byte + 32) as char))
                .copied()
                .unwrap_or(0.0),

            b' ' => 13.0,

            b'.' | b',' | b'\'' | b'!' | b'?' => 1.0,

            0x20..=0x7e => 0.1,

            _ => -20.0,
        })
        .sum()
}

use log::warn;
use std::{collections::HashMap, sync::LazyLock};

pub fn get_token_ids(phonemes: &str) -> Vec<i64> {
    const VOCAB: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
        let mut map = HashMap::new();

        map.insert(';', 1);
        map.insert(':', 2);
        map.insert(',', 3);
        map.insert('.', 4);
        map.insert('!', 5);
        map.insert('?', 6);
        map.insert('—', 9);
        map.insert('…', 10);
        map.insert('"', 11);
        map.insert('(', 12);
        map.insert(')', 13);
        map.insert('“', 14);
        map.insert('”', 15);
        map.insert(' ', 16);
        map.insert('\u{0303}', 17); // Unicode escape for combining tilde
        map.insert('ʣ', 18);
        map.insert('ʥ', 19);
        map.insert('ʦ', 20);
        map.insert('ʨ', 21);
        map.insert('ᵝ', 22);
        map.insert('\u{AB67}', 23); // Unicode escape
        map.insert('A', 24);
        map.insert('I', 25);
        map.insert('O', 31);
        map.insert('Q', 33);
        map.insert('S', 35);
        map.insert('T', 36);
        map.insert('W', 39);
        map.insert('Y', 41);
        map.insert('ᵊ', 42);
        map.insert('a', 43);
        map.insert('b', 44);
        map.insert('c', 45);
        map.insert('d', 46);
        map.insert('e', 47);
        map.insert('f', 48);
        map.insert('h', 50);
        map.insert('i', 51);
        map.insert('j', 52);
        map.insert('k', 53);
        map.insert('l', 54);
        map.insert('m', 55);
        map.insert('n', 56);
        map.insert('o', 57);
        map.insert('p', 58);
        map.insert('q', 59);
        map.insert('r', 60);
        map.insert('s', 61);
        map.insert('t', 62);
        map.insert('u', 63);
        map.insert('v', 64);
        map.insert('w', 65);
        map.insert('x', 66);
        map.insert('y', 67);
        map.insert('z', 68);
        map.insert('ɑ', 69);
        map.insert('ɐ', 70);
        map.insert('ɒ', 71);
        map.insert('æ', 72);
        map.insert('β', 75);
        map.insert('ɔ', 76);
        map.insert('ɕ', 77);
        map.insert('ç', 78);
        map.insert('ɖ', 80);
        map.insert('ð', 81);
        map.insert('ʤ', 82);
        map.insert('ə', 83);
        map.insert('ɚ', 85);
        map.insert('ɛ', 86);
        map.insert('ɜ', 87);
        map.insert('ɟ', 90);
        map.insert('ɡ', 92);
        map.insert('ɥ', 99);
        map.insert('ɨ', 101);
        map.insert('ɪ', 102);
        map.insert('ʝ', 103);
        map.insert('ɯ', 110);
        map.insert('ɰ', 111);
        map.insert('ŋ', 112);
        map.insert('ɳ', 113);
        map.insert('ɲ', 114);
        map.insert('ɴ', 115);
        map.insert('ø', 116);
        map.insert('ɸ', 118);
        map.insert('θ', 119);
        map.insert('œ', 120);
        map.insert('ɹ', 123);
        map.insert('ɾ', 125);
        map.insert('ɻ', 126);
        map.insert('ʁ', 128);
        map.insert('ɽ', 129);
        map.insert('ʂ', 130);
        map.insert('ʃ', 131);
        map.insert('ʈ', 132);
        map.insert('ʧ', 133);
        map.insert('ʊ', 135);
        map.insert('ʋ', 136);
        map.insert('ʌ', 138);
        map.insert('ɣ', 139);
        map.insert('ɤ', 140);
        map.insert('χ', 142);
        map.insert('ʎ', 143);
        map.insert('ʒ', 147);
        map.insert('ʔ', 148);
        map.insert('ˈ', 156);
        map.insert('ˌ', 157);
        map.insert('ː', 158);
        map.insert('ʰ', 162);
        map.insert('ʲ', 164);
        map.insert('↓', 169);
        map.insert('→', 171);
        map.insert('↗', 172);
        map.insert('↘', 173);
        map.insert('ᵻ', 177);
        map
    });

    let mut tokens = Vec::with_capacity(phonemes.len() + 2);
    tokens.push(0);

    for i in phonemes.chars() {
        match VOCAB.get(&i) {
            Some(t) => {
                tokens.push(*t as _);
            }
            _ => {
                warn!("Unknown phone {}, skipped.", i);
            }
        }
    }

    tokens.push(0);
    tokens
}

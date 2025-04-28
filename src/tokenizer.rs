use {
    log::warn,
    std::{collections::HashMap, sync::LazyLock},
};

const VOCAB_V10: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
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

const VOCAB_V11: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(';', 1);
    map.insert(':', 2);
    map.insert(',', 3);
    map.insert('.', 4);
    map.insert('!', 5);
    map.insert('?', 6);
    map.insert('/', 7);
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
    map.insert('ㄓ', 23);
    map.insert('A', 24);
    map.insert('I', 25);
    map.insert('ㄅ', 30);
    map.insert('O', 31);
    map.insert('ㄆ', 32);
    map.insert('Q', 33);
    map.insert('R', 34);
    map.insert('S', 35);
    map.insert('T', 36);
    map.insert('ㄇ', 37);
    map.insert('ㄈ', 38);
    map.insert('W', 39);
    map.insert('ㄉ', 40);
    map.insert('Y', 41);
    map.insert('ᵊ', 42);
    map.insert('a', 43);
    map.insert('b', 44);
    map.insert('c', 45);
    map.insert('d', 46);
    map.insert('e', 47);
    map.insert('f', 48);
    map.insert('ㄊ', 49);
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
    map.insert('ㄋ', 73);
    map.insert('ㄌ', 74);
    map.insert('β', 75);
    map.insert('ɔ', 76);
    map.insert('ɕ', 77);
    map.insert('ç', 78);
    map.insert('ㄍ', 79);
    map.insert('ɖ', 80);
    map.insert('ð', 81);
    map.insert('ʤ', 82);
    map.insert('ə', 83);
    map.insert('ㄎ', 84);
    map.insert('ㄦ', 85);
    map.insert('ɛ', 86);
    map.insert('ɜ', 87);
    map.insert('ㄏ', 88);
    map.insert('ㄐ', 89);
    map.insert('ɟ', 90);
    map.insert('ㄑ', 91);
    map.insert('ɡ', 92);
    map.insert('ㄒ', 93);
    map.insert('ㄔ', 94);
    map.insert('ㄕ', 95);
    map.insert('ㄗ', 96);
    map.insert('ㄘ', 97);
    map.insert('ㄙ', 98);
    map.insert('月', 99);
    map.insert('ㄚ', 100);
    map.insert('ɨ', 101);
    map.insert('ɪ', 102);
    map.insert('ʝ', 103);
    map.insert('ㄛ', 104);
    map.insert('ㄝ', 105);
    map.insert('ㄞ', 106);
    map.insert('ㄟ', 107);
    map.insert('ㄠ', 108);
    map.insert('ㄡ', 109);
    map.insert('ɯ', 110);
    map.insert('ɰ', 111);
    map.insert('ŋ', 112);
    map.insert('ɳ', 113);
    map.insert('ɲ', 114);
    map.insert('ɴ', 115);
    map.insert('ø', 116);
    map.insert('ㄢ', 117);
    map.insert('ɸ', 118);
    map.insert('θ', 119);
    map.insert('œ', 120);
    map.insert('ㄣ', 121);
    map.insert('ㄤ', 122);
    map.insert('ɹ', 123);
    map.insert('ㄥ', 124);
    map.insert('ɾ', 125);
    map.insert('ㄖ', 126);
    map.insert('ㄧ', 127);
    map.insert('ʁ', 128);
    map.insert('ɽ', 129);
    map.insert('ʂ', 130);
    map.insert('ʃ', 131);
    map.insert('ʈ', 132);
    map.insert('ʧ', 133);
    map.insert('ㄨ', 134);
    map.insert('ʊ', 135);
    map.insert('ʋ', 136);
    map.insert('ㄩ', 137);
    map.insert('ʌ', 138);
    map.insert('ɣ', 139);
    map.insert('ㄜ', 140);
    map.insert('ㄭ', 141);
    map.insert('χ', 142);
    map.insert('ʎ', 143);
    map.insert('十', 144);
    map.insert('压', 145);
    map.insert('言', 146);
    map.insert('ʒ', 147);
    map.insert('ʔ', 148);
    map.insert('阳', 149);
    map.insert('要', 150);
    map.insert('阴', 151);
    map.insert('应', 152);
    map.insert('用', 153);
    map.insert('又', 154);
    map.insert('中', 155);
    map.insert('ˈ', 156);
    map.insert('ˌ', 157);
    map.insert('ː', 158);
    map.insert('穵', 159);
    map.insert('外', 160);
    map.insert('万', 161);
    map.insert('ʰ', 162);
    map.insert('ʲ', 164);
    map.insert('为', 165);
    map.insert('文', 166);
    map.insert('瓮', 167);
    map.insert('我', 168);
    map.insert('3', 169);
    map.insert('5', 170);
    map.insert('1', 171);
    map.insert('2', 172);
    map.insert('4', 173);
    map.insert('元', 175);
    map.insert('云', 176);
    map.insert('ᵻ', 177);
    map
});

pub fn get_token_ids(phonemes: &str, v11: bool) -> Vec<i64> {
    let mut tokens = Vec::with_capacity(phonemes.len() + 2);
    tokens.push(0);

    for i in phonemes.chars() {
        let v = if v11 {
            VOCAB_V11.get(&i).map(|i| *i)
        } else {
            VOCAB_V10.get(&i).map(|i| *i)
        };
        match v {
            Some(t) => {
                tokens.push(t as _);
            }
            _ => {
                warn!("Unknown phone {}, skipped.", i);
            }
        }
    }

    tokens.push(0);
    tokens
}

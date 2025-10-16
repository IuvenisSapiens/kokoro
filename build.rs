use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

fn main() {
    const SRC: &str = "src/transcription/en_ipa.c";

    cc::Build::new().file(SRC).compile("es");

    println!("cargo:rerun-if-changed={}", SRC);

    // 处理拼音数据文件，生成拼音字典文件
    write_pinyin_dict(
        "dict/pinyin.dict",
        convert_data_file("data/large_pinyin.txt"),
    );

    println!("cargo:rerun-if-changed=data/large_pinyin.txt");
}

/// 带声调的拼音字符到无声调字符和声调数字的映射表
///
/// 格式：(带声调字符, (无声调字符, 声调数字))
static VOWELS: [(char, (char, u8)); 28] = [
    // a 系列
    ('ā', ('a', 1)),
    ('á', ('a', 2)),
    ('ǎ', ('a', 3)),
    ('à', ('a', 4)),
    // e 系列
    ('ē', ('e', 1)),
    ('é', ('e', 2)),
    ('ě', ('e', 3)),
    ('è', ('e', 4)),
    // i 系列
    ('ī', ('i', 1)),
    ('í', ('i', 2)),
    ('ǐ', ('i', 3)),
    ('ì', ('i', 4)),
    // o 系列
    ('ō', ('o', 1)),
    ('ó', ('o', 2)),
    ('ǒ', ('o', 3)),
    ('ò', ('o', 4)),
    // u 系列
    ('ū', ('u', 1)),
    ('ú', ('u', 2)),
    ('ǔ', ('u', 3)),
    ('ù', ('u', 4)),
    // ü 系列
    ('ǖ', ('ü', 1)),
    ('ǘ', ('ü', 2)),
    ('ǚ', ('ü', 3)),
    ('ǜ', ('ü', 4)),
    // v 系列 (v 是 ü 的简写形式)
    ('ǖ', ('v', 1)),
    ('ǘ', ('v', 2)),
    ('ǚ', ('v', 3)),
    ('ǜ', ('v', 4)),
];

/// 将带声调的拼音转换为无声调拼音加数字声调格式
///
/// # 参数
/// * `tone_map`: 声调映射表
/// * `pinyin`: 待转换的拼音字符串，可能包含多个空格分隔的拼音
///
/// # 返回值
/// 转换后的拼音字符串，格式为 "pin1 yin1"（无声调字母+数字声调）
fn convert_pinyin(tone_map: &HashMap<char, (char, u8)>, pinyin: &str) -> String {
    pinyin
        .split(" ")
        .map(|i| {
            // 遍历拼音中的每个字符
            for char in i.chars() {
                // 如果字符是带声调的拼音字母
                if let Some((base_char, tone)) = tone_map.get(&char) {
                    // 替换带声调的字符为无声调字符，并添加数字声调
                    return format!(
                        "{}{}",
                        i.replace(&char.to_string(), &base_char.to_string()),
                        tone
                    );
                }
            }
            // 如果没有找到带声调的字符，则原样返回
            i.to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// 转换拼音数据文件
///
/// # 参数
/// * `path`: 数据文件路径
///
/// # 返回值
/// 迭代器，产生键值对 (拼音, 音调格式的拼音)
fn convert_data_file<P: AsRef<Path>>(path: P) -> impl IntoIterator<Item = (String, String)> {
    // 打开数据文件
    let file = File::open(path).expect("无法打开拼音数据文件");
    let reader = BufReader::new(file);

    // 创建声调映射表
    let tone_map = HashMap::from(VOWELS);

    // 逐行读取文件并处理
    reader.lines().flat_map(move |line| {
        // 处理读取到的行
        if let Ok(line) = line {
            // 跳过空行、注释行和不包含冒号的行
            if !line.trim().is_empty() && !line.starts_with('#') && line.contains(':') {
                // 分割键值对
                let (key, value) = line.split_once(':').unwrap();
                // 返回处理后的键值对
                Some((key.to_string(), convert_pinyin(&tone_map, value.trim())))
            } else {
                None
            }
        } else {
            None
        }
    })
}

/// 写入拼音字典文件
///
/// # 参数
/// * `p`: 输出文件路径
/// * `data`: 要写入的键值对数据
fn write_pinyin_dict<P: AsRef<Path>>(p: P, data: impl IntoIterator<Item = (String, String)>) {
    // 创建输出文件
    let mut file = File::create(p).expect("无法创建拼音字典文件");
    let mut writer = BufWriter::new(&mut file);

    // 写入所有数据
    for (key, value) in data {
        writer
            .write_fmt(format_args!("{} {}\n", key, value))
            .expect("写入拼音字典文件失败");
    }

    // 确保所有数据都被写入
    writer.flush().expect("刷新缓冲区失败");
}

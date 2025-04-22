/// 文本到国际音标的转换
use super::{PinyinError, pinyin_to_ipa};
use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese};
#[cfg(feature = "use-cmudict")]
use cmudict_fast::{Cmudict, Error as CmudictError};
use pinyin::ToPinyin;
use regex::{Captures, Error as RegexError, Regex};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub enum G2PError {
    #[cfg(feature = "use-cmudict")]
    CmudictError(CmudictError),
    EnptyData,
    #[cfg(not(feature = "use-cmudict"))]
    Nul(std::ffi::NulError),
    Pinyin(PinyinError),
    Regex(RegexError),
    #[cfg(not(feature = "use-cmudict"))]
    Utf8(std::str::Utf8Error),
}

impl Display for G2PError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "G2PError: ")?;
        match self {
            #[cfg(feature = "use-cmudict")]
            Self::CmudictError(e) => Display::fmt(e, f),
            Self::EnptyData => Display::fmt("EmptyData", f),
            #[cfg(not(feature = "use-cmudict"))]
            Self::Nul(e) => Display::fmt(e, f),
            Self::Pinyin(e) => Display::fmt(e, f),
            Self::Regex(e) => Display::fmt(e, f),
            #[cfg(not(feature = "use-cmudict"))]
            Self::Utf8(e) => Display::fmt(e, f),
        }
    }
}

impl Error for G2PError {}

impl From<PinyinError> for G2PError {
    fn from(value: PinyinError) -> Self {
        Self::Pinyin(value)
    }
}

impl From<RegexError> for G2PError {
    fn from(value: RegexError) -> Self {
        Self::Regex(value)
    }
}

#[cfg(feature = "use-cmudict")]
impl From<CmudictError> for G2PError {
    fn from(value: CmudictError) -> Self {
        Self::CmudictError(value)
    }
}

#[cfg(not(feature = "use-cmudict"))]
impl From<std::ffi::NulError> for G2PError {
    fn from(value: std::ffi::NulError) -> Self {
        Self::Nul(value)
    }
}

#[cfg(not(feature = "use-cmudict"))]
impl From<std::str::Utf8Error> for G2PError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

fn retone(p: &str) -> String {
    let chars: Vec<char> = p.chars().collect();
    let mut result = String::with_capacity(p.len());
    let mut i = 0;

    while i < chars.len() {
        match () {
            // 三声调优先处理
            _ if i + 2 < chars.len()
                && chars[i] == '˧'
                && chars[i + 1] == '˩'
                && chars[i + 2] == '˧' =>
            {
                result.push('↓');
                i += 3;
            }
            // 二声调
            _ if i + 1 < chars.len() && chars[i] == '˧' && chars[i + 1] == '˥' => {
                result.push('↗');
                i += 2;
            }
            // 四声调
            _ if i + 1 < chars.len() && chars[i] == '˥' && chars[i + 1] == '˩' => {
                result.push('↘');
                i += 2;
            }
            // 一声调
            _ if chars[i] == '˥' => {
                result.push('→');
                i += 1;
            }
            // 组合字符替换（ɻ̩ 和 ɱ̩）
            _ if i + 1 < chars.len()
                && (
                    (chars[i] == '\u{027B}' && chars[i+1] == '\u{0329}') ||  // ɻ̩
                (chars[i] == '\u{0271}' && chars[i+1] == '\u{0329}')
                    // ɱ̩
                ) =>
            {
                result.push('ɨ');
                i += 2;
            }
            // 默认情况
            _ => {
                result.push(chars[i]);
                i += 1;
            }
        }
    }

    assert!(
        !result.contains('\u{0329}'),
        "Unexpected combining mark in: {}",
        result
    );
    result
}

fn py2ipa(py: &str) -> Result<String, G2PError> {
    pinyin_to_ipa(py)?
        .first()
        .map_or(Err(G2PError::EnptyData), |i| {
            Ok(i.iter().map(|i| retone(i)).collect::<String>())
        })
}

fn word2ipa_zh(word: &str) -> Result<String, G2PError> {
    let iter = word.chars().map(|i| match i.to_pinyin() {
        None => Ok(i.to_string()),
        Some(p) => py2ipa(p.with_tone_num_end()),
    });

    let mut result = String::new();
    for i in iter {
        result.push_str(&i?);
    }
    Ok(result)
}

#[cfg(feature = "use-cmudict")]
fn word2ipa_en(word: &str) -> Result<String, G2PError> {
    use super::{arpa_to_ipa, letters_to_ipa};
    use std::{
        io::{Error as IoError, ErrorKind},
        str::FromStr,
        sync::LazyLock,
    };

    fn get_cmudict<'a>() -> Result<&'a Cmudict, CmudictError> {
        static CMUDICT: LazyLock<Result<Cmudict, CmudictError>> =
            LazyLock::new(|| Cmudict::from_str(include_str!("../dict/cmudict.dict")));
        CMUDICT.as_ref().map_err(|i| match i {
            CmudictError::IoErr(e) => CmudictError::IoErr(IoError::new(ErrorKind::Other, e)),
            CmudictError::InvalidLine(e) => CmudictError::InvalidLine(*e),
            CmudictError::RuleParseError(e) => CmudictError::RuleParseError(e.clone()),
        })
    }

    let Some(rules) = get_cmudict()?.get(word) else {
        return Ok(letters_to_ipa(word));
    };
    if rules.is_empty() {
        return Ok(word.to_owned());
    }
    let i = rand::random_range(0..rules.len());
    let result = rules[i]
        .pronunciation()
        .iter()
        .map(|i| arpa_to_ipa(&i.to_string()).unwrap_or_default())
        .collect::<String>();
    Ok(result)
}

#[cfg(not(feature = "use-cmudict"))]
fn word2ipa_en(word: &str) -> Result<String, G2PError> {
    use std::{
        ffi::{CStr, CString, c_char},
        sync::Once,
    };

    unsafe extern "C" {
        fn TextToPhonemes(text: *const c_char) -> *const ::std::os::raw::c_char;
        fn Initialize(data_dictlist: *const c_char);
    }

    unsafe {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            static DATA: &[u8] = include_bytes!("../dict/espeak.dict");
            Initialize(DATA.as_ptr() as _);
        });

        let word = CString::new(word.to_lowercase())?.into_raw() as *const c_char;
        let res = TextToPhonemes(word);
        Ok(CStr::from_ptr(res).to_str()?.to_string())
    }
}

fn to_half_shape(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2); // 预分配合理空间
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // 处理需要后看的情况
            '«' | '《' => result.push_str("“"),
            '»' | '》' => result.push_str("”"),
            '（' => result.push_str("("),
            '）' => result.push_str(")"),
            // 简单替换规则
            '、' | '，' => result.push_str(","),
            '。' => result.push_str("."),
            '！' => result.push_str("!"),
            '：' => result.push_str(":"),
            '；' => result.push_str(";"),
            '？' => result.push_str("?"),
            // 默认字符
            _ => result.push(c),
        }
    }

    // 清理多余空格并返回
    result
}

fn num_repr(text: &str) -> Result<String, G2PError> {
    let regex = Regex::new(r#"\d+(\.\d+)?"#)?;
    Ok(regex
        .replace(text, |caps: &Captures| {
            let text = &caps[0];
            if let Ok(num) = text.parse::<f64>() {
                num.to_chinese(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    ChineseCountMethod::Low,
                )
                .map_or(text.to_owned(), |i| i)
            } else if let Ok(num) = text.parse::<i64>() {
                num.to_chinese(
                    ChineseVariant::Traditional,
                    ChineseCase::Lower,
                    ChineseCountMethod::Low,
                )
                .map_or(text.to_owned(), |i| i)
            } else {
                text.to_owned()
            }
        })
        .to_string())
}

pub fn g2p(text: &str) -> Result<String, G2PError> {
    let text = num_repr(&text)?;
    let sentence_pattern = Regex::new(
        r#"([\u4E00-\u9FFF]+)|([，。：·？、！《》（）【】〖〗〔〕“”‘’〈〉…—　]+)|([\u0000-\u00FF]+)+"#,
    )?;
    let en_word_pattern = Regex::new("\\w+|\\W+")?;
    let jieba = jieba_rs::Jieba::new();
    let mut result = String::new();
    for i in sentence_pattern.captures_iter(&text) {
        match (i.get(1), i.get(2), i.get(3)) {
            (Some(text), _, _) => {
                let text = to_half_shape(text.as_str());
                for i in jieba.cut(&text, true) {
                    result.push_str(&word2ipa_zh(i)?);
                    result.push(' ');
                }
            }
            (_, Some(text), _) => {
                let text = to_half_shape(text.as_str());
                result = result.trim_end().to_string();
                result.push_str(&text);
                result.push(' ');
            }
            (_, _, Some(text)) => {
                for i in en_word_pattern.captures_iter(text.as_str()) {
                    let c = (&i[0]).chars().nth(0).unwrap_or_default();
                    if c == '\''
                        || c == '_'
                        || c == '-'
                        || c <= 'z' && c >= 'a'
                        || c <= 'Z' && c >= 'A'
                    {
                        let i = &i[0];
                        if result
                            .trim_end()
                            .ends_with(|c| c == '.' || c == ',' || c == '!' || c == '?')
                            && !result.ends_with(' ')
                        {
                            result.push(' ');
                        }
                        result.push_str(&word2ipa_en(i)?);
                    } else if c == ' ' && result.ends_with(' ') {
                        result.push_str((&i[0]).trim_start());
                    } else {
                        result.push_str(&i[0]);
                    }
                }
            }
            _ => (),
        };
    }

    Ok(result.trim().to_string())
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "use-cmudict"))]
    #[test]
    fn test_word2ipa_en() -> Result<(), super::G2PError> {
        use super::word2ipa_en;

        // println!("{:?}", espeak_rs::text_to_phonemes("days", "en", None, true, false));
        assert_eq!("kjˌuːkjˈuː", word2ipa_en("qq")?);
        assert_eq!("həlˈəʊ", word2ipa_en("hello")?);
        assert_eq!("wˈɜːld", word2ipa_en("world")?);
        assert_eq!("ˈapəl", word2ipa_en("apple")?);
        assert_eq!("tʃˈɪldɹɛn", word2ipa_en("children")?);
        assert_eq!("ˈaʊə", word2ipa_en("hour")?);
        assert_eq!("dˈeɪz", word2ipa_en("days")?);

        Ok(())
    }
}

use crate::{g2p, get_model, get_token_ids, get_voice, KokoroError};

use ndarray::Array;
use ort::inputs;
use std::time::{Duration, SystemTime};

/// 语音合成函数
///
/// 该函数接受语音名称、文本内容和速度参数，并返回合成后的音频数据和合成所花费的时间。
///
/// # 参数
///
/// * `voice_name` - 语音名称，用于选择要合成的语音。
/// * `text` - 要合成的文本内容。
/// * `speed` - 合成速度，用于调整合成音频的速度。
///
/// # 返回值
///
/// 返回一个包含合成音频数据和合成所花费时间的元组。
///
/// # 错误处理
///
/// 如果合成过程中出现错误，将返回一个`KokoroError`类型的错误。
///
/// # 示例
///
/// ```rust
/// use kokoro::synth;
///
/// #[tokio::main]
/// async fn main() {
///     if let Ok((audio, took)) = synth("am_puck", "你好，我们是一群追逐梦想的人。", 1.0).await {
///         println!("Synth took: {:?}", took);
///     }
/// }
/// ```
///
/// # 注意
///
/// 请确保在运行此函数之前已经正确加载了模型和语音数据。
///
/// # 错误处理
///
/// 如果合成过程中出现错误，将返回一个`KokoroError`类型的错误。
pub async fn synth<S: AsRef<str>>(
    voice_name: S,
    text: S,
    speed: f32,
) -> Result<(Vec<f32>, Duration), KokoroError> {
    let phonemes = g2p(text.as_ref())?;
    let phonemes = get_token_ids(&phonemes);
    let phonemes = Array::from_shape_vec((1, phonemes.len()), phonemes)?;
    let pack = get_voice(voice_name).await?;
    let ref_s = pack[phonemes.len() - 1]
        .first()
        .map(|i| i.clone())
        .unwrap_or_default();

    let style = Array::from_shape_vec((1, ref_s.len()), ref_s)?;
    let speed = Array::from_vec(vec![speed]);

    let model = get_model().await?;
    let t = SystemTime::now();
    let kokoro_output = model
        .run_async(inputs![
            "tokens" => phonemes.view(),
            "style" => style.view(),
            "speed" => speed.view(),
        ]?)?
        .await?;
    let elapsed = t.elapsed()?;
    if let Some(audio) = kokoro_output["audio"]
        .try_extract_tensor::<f32>()?
        .as_slice()
    {
        return Ok((audio.to_owned(), elapsed));
    }

    Err(KokoroError::SynthFailed(elapsed))
}

use crate::{g2p, get_model, get_token_ids, get_voice, KokoroError};

use ndarray::Array;
use ort::inputs;
use std::time::{Duration, SystemTime};

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

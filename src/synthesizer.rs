use {
    crate::{KokoroError, Voice, g2p, get_token_ids},
    ndarray::Array,
    ort::{inputs, session::Session},
    std::{
        sync::Weak,
        time::{Duration, SystemTime},
    },
};

async fn synth_v10<'a, P, S>(
    model: Weak<Session>,
    phonemes: S,
    pack: P,
    speed: f32,
) -> Result<(Vec<f32>, Duration), KokoroError>
where
    P: AsRef<Vec<Vec<Vec<f32>>>>,
    S: AsRef<str>,
{
    let phonemes = get_token_ids(phonemes.as_ref(), false);
    let phonemes = Array::from_shape_vec((1, phonemes.len()), phonemes)?;
    let ref_s = pack.as_ref()[phonemes.len() - 1]
        .first()
        .map(|i| i.clone())
        .unwrap_or_default();

    let style = Array::from_shape_vec((1, ref_s.len()), ref_s)?;
    let speed = Array::from_vec(vec![speed]);
    let model = model.upgrade().ok_or(KokoroError::ModelReleased)?;
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

async fn synth_v11<P, S>(
    model: Weak<Session>,
    phonemes: S,
    pack: P,
    speed: i32,
) -> Result<(Vec<f32>, Duration), KokoroError>
where
    P: AsRef<Vec<Vec<Vec<f32>>>>,
    S: AsRef<str>,
{
    let phonemes = get_token_ids(phonemes.as_ref(), true);
    let phonemes = Array::from_shape_vec((1, phonemes.len()), phonemes)?;
    let ref_s = pack.as_ref()[phonemes.len() - 1]
        .first()
        .map(|i| i.clone())
        .unwrap_or_default();

    let style = Array::from_shape_vec((1, ref_s.len()), ref_s)?;
    let speed = Array::from_vec(vec![speed]);
    let model = model.upgrade().ok_or(KokoroError::ModelReleased)?;
    let t = SystemTime::now();
    let kokoro_output = model
        .run_async(inputs![
            "input_ids" => phonemes.view(),
            "style" => style.view(),
            "speed" => speed.view(),
        ]?)?
        .await?;
    let elapsed = t.elapsed()?;
    if let (Some(audio), Some(duration)) = (
        kokoro_output["waveform"]
            .try_extract_tensor::<f32>()?
            .as_slice(),
        kokoro_output["duration"]
            .try_extract_tensor::<i64>()?
            .as_slice(),
    ) {
        let _ = dbg!(duration.len());
        return Ok((audio.to_owned(), elapsed));
    }

    Err(KokoroError::SynthFailed(elapsed))
}

pub(super) async fn synth<'a, P, S>(
    model: Weak<Session>,
    text: S,
    pack: P,
    voice: Voice,
) -> Result<(Vec<f32>, Duration), KokoroError>
where
    P: AsRef<Vec<Vec<Vec<f32>>>>,
    S: AsRef<str>,
{
    let phonemes = g2p(text.as_ref(), voice.is_v11_supported())?;
    #[cfg(debug_assertions)]
    println!("{}", phonemes);
    match voice {
        v if v.is_v11_supported() => synth_v11(model, phonemes, pack, v.get_speed_v11()?).await,
        v if v.is_v10_supported() => synth_v10(model, phonemes, pack, v.get_speed_v10()?).await,
        v => Err(KokoroError::VoiceVersionInvalid(v.get_name().to_owned())),
    }
}

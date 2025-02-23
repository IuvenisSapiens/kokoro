use kokoro::{get_voice_names, load, synth};
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load("kokoro-v1.0.int8.onnx", "voices.bin").await?;
    let voice_names = get_voice_names().await?;
    for i in voice_names.iter() {
        println!("{i}");
    }

    let (audio, took) = synth(
        "am_puck",
        "你好，我们是一群追逐梦想的人。我正在使用qq。",
        1.0,
    )
    .await?;
    println!("Synth took: {:?}", took);
    play_sound(&audio);
    Ok(())
}

fn play_sound(data: &[f32]) {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let player = Sink::try_new(&handle).unwrap();
    player.append(SamplesBuffer::new(1, 24000, data));
    player.sleep_until_end()
}

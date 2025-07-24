use kokoro_tts::{KokoroTts, Voice};
use rodio::{OutputStreamBuilder, Sink, buffer::SamplesBuffer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tts = KokoroTts::new("kokoro-v1.1-zh.onnx", "voices-v1.1-zh.bin").await?;
    let (audio, took) = tts
        .synth(
            "Hello, world!你好，我们是一群追逐梦想的人。我正在使用qq。",
            Voice::Zm045(1),
        )
        .await?;
    println!("Synth took: {:?}", took);
    play_sound(&audio);
    Ok(())
}

fn play_sound(data: &[f32]) {
    let output_stream_builder = OutputStreamBuilder::from_default_device().unwrap();
    let output_stream = output_stream_builder.open_stream().unwrap();
    let stream_handle = output_stream.mixer();
    let player = Sink::connect_new(&stream_handle);

    player.append(SamplesBuffer::new(1, 24000, data));
    player.sleep_until_end()
}

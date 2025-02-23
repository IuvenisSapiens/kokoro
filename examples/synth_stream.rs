use futures::StreamExt;
use kokoro_tts::{get_voice_names, load, start_synth_session};
use rodio::{OutputStream, Sink, buffer::SamplesBuffer};
use std::sync::Arc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load("kokoro-v1.0.int8.onnx", "voices.bin").await?;
    let voice_names = get_voice_names().await?;
    for i in voice_names.iter() {
        println!("{i}");
    }

    let (mut sink, mut stream) = start_synth_session("zf_xiaoxiao", 1.1);
    sink.synth("hello world.").await?;
    sink.synth("你好，我们是一群追逐梦想的人。").await?;
    sink.set_voice("zm_yunxi");
    sink.set_speed(1.2);
    sink.synth("我正在使用qq。").await?;
    sink.set_voice("zm_yunyang");
    sink.synth("今天天气如何？").await?;
    sink.set_voice("af_sarah");
    sink.synth("你在使用Rust编程语言吗？").await?;

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let player = Arc::new(Sink::try_new(&handle)?);
    let player2 = player.clone();
    tokio::spawn(async move {
        while let Some((audio, took)) = stream.next().await {
            player.append(SamplesBuffer::new(1, 24000, audio));
            println!("Synth took: {:?}", took);
        }
    });

    sleep(Duration::from_secs(20)).await;
    player2.sleep_until_end();
    Ok(())
}

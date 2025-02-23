use crate::{synth, KokoroError};
use futures::{Sink, SinkExt, Stream};
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

struct Request<S> {
    voice_name: S,
    text: S,
    speed: f32,
}

struct Response {
    data: Vec<f32>,
    took: Duration,
}

/// 语音合成流
///
/// 该结构体用于通过流式合成来处理更长的文本。它实现了`Stream` trait，可以用于异步迭代合成后的音频数据。
#[pin_project]
pub struct SynthStream {
    #[pin]
    rx: UnboundedReceiver<Response>,
}

impl Stream for SynthStream {
    type Item = (Vec<f32>, Duration);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.project().rx)
            .poll_recv(cx)
            .map(|i| match i {
                None => None,
                Some(Response { data, took }) => Some((data, took)),
            })
    }
}

/// 语音合成发送端
///
/// 该结构体用于发送语音合成请求。它实现了`Sink` trait，可以用于异步发送合成请求。
#[pin_project]
pub struct SynthSink<S> {
    tx: UnboundedSender<Request<S>>,
    voice_name: S,
    speed: f32,
}

impl<S: Clone> SynthSink<S> {
    /// 设置语音名称
    ///
    /// 该方法用于设置要合成的语音名称。
    ///
    /// # 参数
    ///
    /// * `voice_name` - 语音名称，用于选择要合成的语音。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use kokoro::start_synth_session;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (mut sink, _) = start_synth_session("zf_xiaoxiao", 1.1);
    ///     sink.set_voice("zm_yunxi");
    /// }
    /// ```
    ///
    pub fn set_voice(&mut self, voice_name: S) {
        self.voice_name = voice_name
    }

    /// 设置合成速度
    ///
    /// 该方法用于设置合成速度，用于调整合成音频的速度。
    ///
    /// # 参数
    ///
    /// * `speed` - 合成速度，用于调整合成音频的速度。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use kokoro::start_synth_session;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (mut sink, _) = start_synth_session("zf_xiaoxiao", 1.1);
    ///     sink.set_speed(1.2);
    /// }
    /// ```
    ///
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed
    }

    /// 发送合成请求
    ///
    /// 该方法用于发送语音合成请求。
    ///
    /// # 参数
    ///
    /// * `text` - 要合成的文本内容。
    ///
    /// # 返回值
    ///
    /// 如果发送成功，将返回`Ok(())`；如果发送失败，将返回一个`KokoroError`类型的错误。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use kokoro::start_synth_session;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (mut sink, _) = start_synth_session("zf_xiaoxiao", 1.1);
    ///     let _ = sink.synth("hello world.").await;
    /// }
    /// ```
    ///
    pub async fn synth(&mut self, text: S) -> Result<(), KokoroError> {
        self.send((self.voice_name.clone(), text, self.speed)).await
    }
}

impl<S> Sink<(S, S, f32)> for SynthSink<S> {
    type Error = KokoroError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(
        self: Pin<&mut Self>,
        (voice_name, text, speed): (S, S, f32),
    ) -> Result<(), Self::Error> {
        self.tx
            .send(Request {
                voice_name,
                text,
                speed,
            })
            .map_err(|e| KokoroError::Send(e.to_string()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

/// 启动语音合成会话
///
/// 该函数用于启动一个语音合成会话，返回一个`SynthSink`和`SynthStream`，用于发送合成请求和接收合成结果。
///
/// # 参数
///
/// * `voice_name` - 语音名称，用于选择要合成的语音。
/// * `speed` - 合成速度，用于调整合成音频的速度。
///
/// # 返回值
///
/// 返回一个包含`SynthSink`和`SynthStream`的元组。
///
/// # 示例
///
/// ```rust
/// use kokoro::start_synth_session;
/// use futures::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let (mut sink, mut stream) = start_synth_session("zf_xiaoxiao", 1.1);
///     let _ = sink.synth("hello world.").await;
///     let _ = sink.synth("你好，我们是一群追逐梦想的人。").await;
///     sink.set_speed(1.2);
///     sink.set_voice("zm_yunyang");
///     let _ = sink.synth("今天天气如何？").await;
///
///     while let Some((audio, took)) = stream.next().await {
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
pub fn start_synth_session<S: AsRef<str> + Send + 'static>(
    voice_name: S,
    speed: f32,
) -> (SynthSink<S>, SynthStream) {
    let (tx, mut rx) = unbounded_channel::<Request<S>>();
    let (tx2, rx2) = unbounded_channel();
    tokio::spawn(async move {
        while let Some(req) = rx.recv().await {
            let (data, took) = synth(req.voice_name, req.text, req.speed).await?;
            tx2.send(Response { data, took })
                .map_err(|e| KokoroError::Send(e.to_string()))?;
        }

        Ok::<_, KokoroError>(())
    });

    (
        SynthSink {
            tx,
            voice_name,
            speed,
        },
        SynthStream { rx: rx2 },
    )
}

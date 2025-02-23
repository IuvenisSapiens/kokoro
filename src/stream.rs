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

#[pin_project]
pub struct SynthSink<S> {
    tx: UnboundedSender<Request<S>>,
    voice_name: S,
    speed: f32,
}

impl<S: Clone> SynthSink<S> {
    pub fn set_voice(&mut self, voice_name: S) {
        self.voice_name = voice_name
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed
    }

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

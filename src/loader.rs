use crate::KokoroError;
use bincode::{config::standard, decode_from_slice};
use ort::{execution_providers::CUDAExecutionProvider, session::Session};
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, LazyLock},
};
use tokio::{fs::read, sync::Mutex};

static VOICES: LazyLock<Mutex<HashMap<String, Vec<Vec<Vec<f32>>>>>> =
    LazyLock::new(Default::default);
static MODEL: LazyLock<Mutex<Option<Arc<Session>>>> = LazyLock::new(Default::default);

pub async fn load<P: AsRef<Path>>(model_path: P, voices_path: P) -> Result<(), KokoroError> {
    let voices = read(voices_path).await?;
    let (voices, _) =
        decode_from_slice::<HashMap<String, Vec<Vec<Vec<f32>>>>, _>(&voices, standard())?;
    let mut lock = VOICES.lock().await;
    *lock = voices;

    let model = Session::builder()?
        .with_execution_providers([CUDAExecutionProvider::default().build()])?
        .commit_from_file(model_path)?;
    MODEL
        .lock()
        .await
        .replace(model.into())
        .map_or(Ok(()), |_| Err(KokoroError::LoadFailed))
}

pub async fn get_voice<S: AsRef<str>>(name: S) -> Result<Vec<Vec<Vec<f32>>>, KokoroError> {
    let lock = VOICES.lock().await;
    lock.get(name.as_ref())
        .ok_or(KokoroError::NotLoaded)
        .map(|i| i.to_owned())
}

pub async fn get_voice_names() -> Result<Vec<String>, KokoroError> {
    Ok(VOICES.lock().await.keys().map(|i| i.to_owned()).collect())
}

pub async fn unload() {
    VOICES.lock().await.clear();
}

pub async fn get_model() -> Result<Arc<Session>, KokoroError> {
    if let Some(s) = MODEL.lock().await.as_ref() {
        return Ok(s.clone());
    }
    Err(KokoroError::NotLoaded)
}

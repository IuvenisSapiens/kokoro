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

/// 加载模型和语音数据
///
/// 该函数用于加载Kokoro模型和语音数据。加载完成后，可以使用`synth`函数进行语音合成。
///
/// # 参数
///
/// * `model_path` - 模型文件的路径。
/// * `voice_data_path` - 语音数据文件的路径。
///
/// # 返回值
///
/// 如果加载成功，将返回`Ok(())`；如果加载失败，将返回一个`KokoroError`类型的错误。
///
/// # 示例
///
/// ```rust
/// use kokoro::load;
///
/// #[tokio::main]
/// async fn main() {
///     let _ = load("kokoro-v1.0.int8.onnx", "voices.bin").await;
/// }
/// ```
///
/// # 注意
///
/// 请确保在运行此函数之前模型文件和语音数据文件已经存在于指定路径。
///
/// # 错误处理
///
/// 如果加载过程中出现错误，将返回一个`KokoroError`类型的错误。
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

/// 获取语音名称列表
///
/// 该函数用于获取可用的语音名称列表。
///
/// # 返回值
///
/// 返回一个包含语音名称的`Vec<String>`。
///
/// # 示例
///
/// ```rust
/// use kokoro::get_voice_names;
///
/// #[tokio::main]
/// async fn main() {
///     if let Ok(voice_names) = get_voice_names().await {
///         println!("Voice names: {:?}", voice_names);
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
/// 如果获取过程中出现错误，将返回一个`KokoroError`类型的错误。
pub async fn get_voice_names() -> Result<Vec<String>, KokoroError> {
    Ok(VOICES.lock().await.keys().map(|i| i.to_owned()).collect())
}

/// 卸载模型和语音数据
///
/// 该函数用于卸载已经加载的Kokoro模型和语音数据。卸载完成后，需要重新加载模型和语音数据才能进行语音合成。
///
/// # 返回值
///
/// 如果卸载成功，将返回`Ok(())`；如果卸载失败，将返回一个`KokoroError`类型的错误。
///
/// # 示例
///
/// ```rust
/// use kokoro::unload;
///
/// #[tokio::main]
/// async fn main() {
///     let _ = unload().await;
/// }
/// ```
///
/// # 注意
///
/// 请确保在运行此函数之前已经正确加载了模型和语音数据。
///
/// # 错误处理
///
/// 如果卸载过程中出现错误，将返回一个`KokoroError`类型的错误。
pub async fn unload() -> Result<(), KokoroError> {
    VOICES.lock().await.clear();
    MODEL
        .lock()
        .await
        .take()
        .map_or(Err(KokoroError::NotLoaded), |_| Ok(()))
}

pub(super) async fn get_voice<S: AsRef<str>>(name: S) -> Result<Vec<Vec<Vec<f32>>>, KokoroError> {
    let lock = VOICES.lock().await;
    lock.get(name.as_ref())
        .ok_or(KokoroError::NotLoaded)
        .map(|i| i.to_owned())
}

pub(super) async fn get_model() -> Result<Arc<Session>, KokoroError> {
    if let Some(s) = MODEL.lock().await.as_ref() {
        return Ok(s.clone());
    }
    Err(KokoroError::NotLoaded)
}

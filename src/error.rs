use crate::G2PError;
use bincode::error::DecodeError;
use ndarray::ShapeError;
use ort::Error as OrtError;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    time::SystemTimeError,
};

#[derive(Debug)]
pub enum KokoroError {
    Decode(DecodeError),
    G2P(G2PError),
    Io(IoError),
    ModelReleased,
    Ort(OrtError),
    Send(String),
    Shape(ShapeError),
    SystemTime(SystemTimeError),
    VoiceNotFound(String),
    VoiceVersionInvalid(String),
}

impl Display for KokoroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "KokoroError: ")?;
        match self {
            Self::Decode(e) => Display::fmt(e, f),
            Self::G2P(e) => Display::fmt(e, f),
            Self::Io(e) => Display::fmt(e, f),
            Self::Ort(e) => Display::fmt(e, f),
            Self::ModelReleased => write!(f, "ModelReleased"),
            Self::Send(e) => Display::fmt(e, f),
            Self::Shape(e) => Display::fmt(e, f),
            Self::SystemTime(e) => Display::fmt(e, f),
            Self::VoiceNotFound(name) => write!(f, "VoiceNotFound({})", name),
            Self::VoiceVersionInvalid(msg) => write!(f, "VoiceVersionInvalid({})", msg),
        }
    }
}

impl Error for KokoroError {}

impl From<IoError> for KokoroError {
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}

impl From<DecodeError> for KokoroError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

impl From<G2PError> for KokoroError {
    fn from(value: G2PError) -> Self {
        Self::G2P(value)
    }
}

impl From<ShapeError> for KokoroError {
    fn from(value: ShapeError) -> Self {
        Self::Shape(value)
    }
}

impl From<SystemTimeError> for KokoroError {
    fn from(value: SystemTimeError) -> Self {
        Self::SystemTime(value)
    }
}

// `ort::Error` is parameterized by a recovery type, and some APIs (such as the
// session builder) return `Error<SessionBuilder>`.  Because the builder type is
// private we cannot write a `From` impl specifically for it, but we can implement
// a catch‑all conversion for *any* instantiation.  We convert to our existing
// `Ort` variant by dropping the recovery value and copying the error message.
impl<T> From<ort::Error<T>> for KokoroError {
    fn from(value: ort::Error<T>) -> Self {
        // `OrtError::new` is a constructor that accepts a message string; the
        // wrapped error implements `Display`, so `to_string()` gives us something
        // reasonable.
        KokoroError::Ort(OrtError::new(value.to_string()))
    }
}
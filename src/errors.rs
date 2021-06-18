use serde::ser;

#[derive(Debug, PartialEq, Eq)]
pub enum EncoderError {
    FloatNotSupported,
    NotSupported,
    Custom(String),
}

impl std::fmt::Display for EncoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EncoderError {}

impl ser::Error for EncoderError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        EncoderError::Custom(format!("{}", msg))
    }
}

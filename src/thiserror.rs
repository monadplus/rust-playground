use thiserror::Error;

#[derive(Debug, Error)]
pub enum OuterError {
    #[error("Inner error: {0}")]
    Inner(#[from] InnerError),
}

#[derive(Debug, Error)]
pub enum InnerError {
    #[error("This is a cursed String {0}")]
    IsCurse(&'static str),
}

pub fn inner() -> Result<usize, InnerError> {
    Err(InnerError::IsCurse("Cursed!"))
}

pub fn upper_cast() -> Result<usize, OuterError> {
    let x = inner()?;
    Ok(x)
}

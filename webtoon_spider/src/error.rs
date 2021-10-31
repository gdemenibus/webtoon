use std::{io, result::Result as StdResult};
use thiserror::Error;

pub type Result<T> = StdResult<T, WebtoonSpiderError>;

#[derive(Error, Debug)]
pub enum WebtoonSpiderError {
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("io error")]
    IoError(#[from] io::Error),

    #[error("No image list")]
    NoImageList,

    #[error("No episode list")]
    NoEpisodeList,
}

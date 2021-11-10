use std::path::{Path, PathBuf};

use bytes::Bytes;
use reqwest::{Client, header};
use crate::{BASE_URL, episode, error::Result};

pub async fn download_image(client: &Client, url: &str) -> Result<Bytes> {
    Ok(client
        .get(url)
        .header(header::REFERER, BASE_URL) // Required as webtoon uses referer as anti-hotlink protection
        .send()
        .await?
        .bytes()
        .await?)
}

pub async fn download_episode<P: AsRef<Path>>(client: &Client, episode_images: Vec<String>, dir: P) -> Result<()> {
    Ok(())
}

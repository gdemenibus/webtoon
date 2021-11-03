use bytes::Bytes;
use reqwest::{Client, header};
use crate::{BASE_URL, error::Result};

pub async fn download_image(client: &Client, url: &str) -> Result<Bytes> {
    Ok(client
        .get(url)
        .header(header::REFERER, BASE_URL) // Required as webtoon uses referer as anti-hotlink protection
        .send()
        .await?
        .bytes()
        .await?)
}

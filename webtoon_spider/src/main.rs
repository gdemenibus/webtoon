#[macro_use]
extern crate html5ever;

mod episode;
mod error;
mod html;
mod title;
mod download;
mod url_extractor;

use error::Result;
use reqwest::Client;
use tokio::{fs::File, io::AsyncWriteExt};

const BASE_URL: &str = "https://www.webtoons.com/en";
const COOKIE_PREFIX: &str = "locale=en; needGDPR=true; needCCPA=false; needCOPPA=false; countryCode=NL; timezoneOffset=+2; pagGDPR=true; atGDPR=AD_CONSENT";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let images = fetch_comic(&client, 2616, Some(76)).await.unwrap();
    let first_img = download::download_image(&client, &images[0][0]).await.unwrap();

    let mut file = File::create("./image.jpeg").await.unwrap();
    file.write_all(&*first_img).await.unwrap();
}

async fn fetch_comic(client: &Client, title_no: usize, from_episode: Option<usize>) -> Result<Vec<Vec<String>>> {
    let latest = title::get_latest_episode_number(client, title_no).await?;

    let mut images = Vec::with_capacity(latest);

    let start = from_episode.unwrap_or(1);
    for episode_no in start..=latest {
        let episode_images = episode::fetch_urls_of_episode(client, title_no, episode_no).await?;
        images.push(episode_images);
    }
    
    Ok(images)
}

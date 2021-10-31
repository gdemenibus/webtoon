#[macro_use]
extern crate html5ever;

mod episode;
mod error;

const BASE_URL: &str = "https://www.webtoons.com/en";
const COOKIE_PREFIX: &str = "locale=en; needGDPR=true; needCCPA=false; needCOPPA=false; countryCode=NL; timezoneOffset=+2; pagGDPR=true; atGDPR=AD_CONSENT";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let images = episode::fetch_urls_of_episode(&client, 2616, 2)
        .await
        .unwrap();

    println!("Images: {:?}", images);
}

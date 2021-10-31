#[macro_use]
extern crate html5ever;

mod downloader;
mod error;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    // let client = reqwest::Client::builder().build().unwrap();
    let images = downloader::get_image_urls_of_comic_page(&client, 2616, 2)
        .await
        .unwrap();

    println!("Images: {:?}", images);
}

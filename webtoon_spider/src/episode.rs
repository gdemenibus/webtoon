use bytes::Bytes;

use html5ever::{tendril::Tendril, LocalName, QualName};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use reqwest::{self, header, Client};

use crate::{
    error::{Result, WebtoonSpiderError},
    html::{parse_html, walker},
    BASE_URL, COOKIE_PREFIX,
};

async fn get_html(client: &Client, title_no: usize, episode_no: usize) -> Result<Bytes> {
    let url = format!(
        "{}/fake-category/fake-title/fake-episode/viewer?title_no={}&episode_no={}",
        BASE_URL, title_no, episode_no
    );
    let cookie = format!("{}; rw=w_{}_{}", COOKIE_PREFIX, title_no, episode_no);

    Ok(client
        .get(url)
        .header(header::COOKIE, cookie)
        .send()
        .await?
        .bytes()
        .await?)
}

fn find_image_list(dom: &RcDom) -> Option<Vec<Handle>> {
    let qual_id = QualName::new(None, ns!(), local_name!("id"));
    let qual_val = Tendril::from("_imageList");

    walker(&dom.document, |data| match data {
        NodeData::Element { attrs, .. } => attrs
            .borrow()
            .iter()
            .any(|a| a.name == qual_id && a.value == qual_val),
        _ => false,
    })
    .map(|a| a.children.clone().into_inner())
}

fn to_image_urls(handle: &[Handle]) -> Vec<String> {
    let qual_img = QualName::new(None, ns!(html), local_name!("img"));
    let qual_data_url = QualName::new(None, ns!(), LocalName::from("data-url"));

    handle
        .iter()
        .filter_map(|node| match &node.data {
            NodeData::Element { name, attrs, .. } if name == &qual_img => Some(
                attrs
                    .borrow()
                    .iter()
                    .find(|a| a.name == qual_data_url)?
                    .value
                    .to_string(),
            ),
            _ => None,
        })
        .collect()
}

pub async fn fetch_urls_of_episode(
    client: &Client,
    title_no: usize,
    episode_no: usize,
) -> Result<Vec<String>> {
    let html = get_html(client, title_no, episode_no).await?;
    let dom = parse_html(&html)?;
    let nodes = find_image_list(&dom).ok_or(WebtoonSpiderError::NoImageList)?;
    let images = to_image_urls(&nodes);

    Ok(images)
}

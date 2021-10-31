use std::rc::Rc;

use bytes::Bytes;

use html5ever::{
    parse_document,
    tendril::{Tendril, TendrilSink},
    LocalName, QualName,
};
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom};
use reqwest::{self, header, Client};

use crate::error::{Result, WebtoonSpiderError};

const BASE_URL: &str = "https://www.webtoons.com/en";
const COOKIE_PREFIX: &str = "locale=en; needGDPR=true; needCCPA=false; needCOPPA=false; countryCode=NL; timezoneOffset=+2; pagGDPR=true; atGDPR=AD_CONSENT";

pub async fn get_html(client: &Client, title_no: usize, episode_no: usize) -> Result<Bytes> {
    let url = format!(
        "{}/fake-category/fake-title/fake-episode/viewer?title_no={}&episode_no={}",
        BASE_URL, title_no, episode_no
    );
    let cookie = format!("{}; rw=w_{}_{}", COOKIE_PREFIX, title_no, episode_no);

    let html = client
        .get(url)
        .header(header::COOKIE, cookie)
        .send()
        .await?
        .bytes()
        .await?;
    Ok(html)
}

fn parse_html(bytes: Bytes) -> Result<RcDom> {
    let dom = parse_document(RcDom::default(), html5ever::ParseOpts::default())
        .from_utf8()
        .read_from(&mut &*bytes)?;

    Ok(dom)
}

fn walker(handle: &Handle) -> Option<Handle> {
    let node = handle;

    let qual_id = QualName::new(None, ns!(), local_name!("id"));

    let qual_val = Tendril::from("_imageList");

    if let NodeData::Element { attrs, .. } = &node.data {
        // attrs.borrow().into_iter().filter(|a| a.name == qual_id);
        if attrs
            .borrow()
            .iter()
            .any(|a| a.name == qual_id && a.value == qual_val)
        {
            return Some(node.clone());
        };
    }

    for i in node.children.borrow().iter() {
        let recursive = walker(i);
        if recursive.is_some() {
            return recursive;
        }
    }

    None
}
fn find_image_list(dom: &RcDom) -> Option<Vec<Rc<Node>>> {
    // TODO: Can this be neater?
    walker(&dom.document).map(|a| a.children.clone().into_inner())
}

fn to_image_urls(handle: &[Handle]) -> Vec<String> {
    let qual_img = QualName::new(None, ns!(html), local_name!("img"));
    let qual_data_url = QualName::new(None, ns!(), LocalName::from("data-url"));

    let res = handle
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
        .collect();

    res
}

pub async fn get_image_urls_of_comic_page(
    client: &Client,
    title_no: usize,
    episode_no: usize,
) -> Result<Vec<String>> {
    let html = get_html(client, title_no, episode_no).await?;
    let dom = parse_html(html)?;

    let nodes = find_image_list(&dom).ok_or(WebtoonSpiderError::NoImageList)?;
    Ok(to_image_urls(&nodes))
}

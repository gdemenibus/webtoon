use bytes::Bytes;
use html5ever::{tendril::Tendril, LocalName, QualName};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use reqwest::{header, Client};

use crate::{
    error::{Result, WebtoonSpiderError},
    html::{parse_html, walker},
    BASE_URL, COOKIE_PREFIX,
};

async fn get_html(client: &Client, title_no: usize) -> Result<Bytes> {
    let url = format!(
        "{}/fake-category/fake-title/list?title_no={}",
        BASE_URL, title_no
    );
    let cookie = format!("{}; rw=w_{}_1", COOKIE_PREFIX, title_no);

    Ok(client
        .get(url)
        .header(header::COOKIE, cookie)
        .send()
        .await?
        .bytes()
        .await?)
}

fn find_episode_list(dom: &RcDom) -> Option<Handle> {
    let qual_id = QualName::new(None, ns!(), local_name!("id"));
    let qual_val = Tendril::from("_listUl");

    walker(&dom.document, |data| match data {
        NodeData::Element { attrs, .. } => attrs
            .borrow()
            .iter()
            .any(|a| a.name == qual_id && a.value == qual_val),
        _ => false,
    })
}

fn find_latest_episode(list: Handle) -> Option<usize> {
    let qual_li = QualName::new(None, ns!(html), local_name!("li"));
    let qual_data_episode = QualName::new(None, ns!(), LocalName::from("data-episode-no"));

    let children = list.children.borrow();
    children.iter().find_map(|node| match &node.data {
        NodeData::Element { name, attrs, .. } if name == &qual_li => Some(
            attrs
                .borrow()
                .iter()
                .find(|a| a.name == qual_data_episode)?
                .value
                .parse()
                .ok()?,
        ),
        _ => None,
    })
}

pub async fn get_latest_episode_number(client: &Client, title_no: usize) -> Result<usize> {
    let html = get_html(client, title_no).await?;
    let dom = parse_html(&html)?;
    let list = find_episode_list(&dom).ok_or(WebtoonSpiderError::NoEpisodeList)?;
    let no = find_latest_episode(list).unwrap();
    Ok(no)
}

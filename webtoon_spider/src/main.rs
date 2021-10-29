use std::ops::Deref;

use bytes::Bytes;

#[macro_use]
extern crate html5ever;

use html5ever::{
    parse_document,
    tendril::{Tendril, TendrilSink},
    QualName,
};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use reqwest::{self, header};
use tokio;

async fn request() {
    let url = "https://www.webtoons.com/en/slice-of-life/boyfriends/episode-2/viewer?title_no=2616&episode_no=2";
    let client = reqwest::Client::builder().build().unwrap();

    let res = client.get(url).header(header::COOKIE, "locale=en; needGDPR=true; needCCPA=false; needCOPPA=false; countryCode=NL; timezoneOffset=+2; pagGDPR=true; atGDPR=AD_CONSENT; rw=w_2616_2").send().await.unwrap();
    // println!("body: {:?}", res);
    image_extractor(res.bytes().await.unwrap())
}

fn image_extractor(mut bytes: Bytes) {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut bytes.deref())
        .unwrap();
    // dom.document
    let node = walker(&dom.document).unwrap();

    processes_node_to_image_array(node);
}
fn walker(handle: &Handle) -> Option<Handle> {
    let node = handle;

    let qual_id = QualName::new(None, ns!(), local_name!("id"));

    let qual_val = Tendril::from("_imageList");

    match &node.data {
        el @ NodeData::Element { name, attrs, .. } => {
            // attrs.borrow().into_iter().filter(|a| a.name == qual_id);
            if let Some(a) = attrs
                .borrow()
                .iter()
                .find(|a| a.name == qual_id && a.value == qual_val)
            {
                dbg!(a);
                return Some(node.clone());
            };
        }
        _ => {}
    }

    for i in node.children.borrow().iter() {
        let recursive = walker(i);
        if recursive.is_some() {
            return recursive;
        }
    }

    return None;
}

fn processes_node_to_image_array(handle: Handle) -> Vec<String> {
    let qual_img = QualName::new(None, ns!(html), local_name!("img"));

    handle.children.borrow().iter().filter(|node| match &node.data {
        NodeData::Element { name, ..} => name == &qual_img,
        _ => false
    }).for_each(|a| { dbg!(a); });

    Vec::new()
}
#[tokio::main]
async fn main() {
    request().await
}

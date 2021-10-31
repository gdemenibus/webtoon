use std::convert::identity;

use bytes::Bytes;

use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use crate::error::Result;

pub fn parse_html(bytes: &Bytes) -> Result<RcDom> {
    Ok(
        parse_document(RcDom::default(), html5ever::ParseOpts::default())
            .from_utf8()
            .read_from(&mut &**bytes)?,
    )
}

pub fn walker<F>(handle: &Handle, f: F) -> Option<Handle>
where
    F: Copy + FnOnce(&NodeData) -> bool,
{
    if f(&handle.data) {
        Some(handle.clone())
    } else {
        handle
            .children
            .borrow()
            .iter()
            .map(|i| walker(i, f))
            .find_map(identity)
    }
}

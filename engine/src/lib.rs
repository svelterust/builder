use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Element {
    tag: String,
    text: Option<String>,
    children: Option<Vec<Element>>,
}

#[wasm_bindgen]
pub fn render_element(element: Element) -> String {
    let children_html = element.children.map_or(String::new(), |children| {
        children.into_iter().map(render_element).collect::<String>()
    });
    format!(
        "<{}>{}{}</{}>",
        element.tag,
        element.text.unwrap_or_default(),
        children_html,
        element.tag
    )
}

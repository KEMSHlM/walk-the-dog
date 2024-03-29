use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ Document, Window, HtmlCanvasElement, Response };
use anyhow::{ anyhow, Result };
use std::future::Future;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen_futures::JsFuture;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("No Document Found"))
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Document Found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err( |element| anyhow!("Error converting {:?} to HtmlCanvasElement", element))
}

pub fn context() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err( |js_value| anyhow!("Error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err( |element| {
            anyhow!("Error converting {:#?} to CanvasRenderingContext2d", element)
        })
}

pub fn spawn_local<F> (future: F)
    where
        F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
    JsFuture::from(window()?.fetch_with_str(resource))
        .await
        .map_err(|err| anyhow!("Error fetching {:#?}", err))
}

pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
    let resp_value = fetch_with_str(json_path).await?;
    let resp: Response = resp_value
        .dyn_into()
        .map_err(|element| anyhow!("Error convertiong {:#?} to Response", element))?;


    JsFuture::from(
        resp.json()
            .map_err(|err| anyhow!("Could not get JSON from response {:#?}", err))?
    )
    .await
    .map_err(|err| anyhow!("Error fetching Json {:#?}", err))

}

pub fn new_image() -> Result<HtmlImageElement> {
    HtmelImageElement::new()
        .map_err(|err| anyhow!("Could not create new image {:#?}", err))

}

pub fn closure_once<F, A, R> (fn_once: F) -> Closure<F::FnMnt>
    where
        F: 'static + WasmClosure<A, R>,
{
    Closure::once(fn_once)
}



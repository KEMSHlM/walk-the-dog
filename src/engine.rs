use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::sync::Mutex;


#[macro_use]
mod browser;

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image();

    let (complet_tx, complet_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(success_tx)));
    let error_tx = Rc::clone(&success_tx);
    let success_callback = brwoser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok()
            .and_then(|mut opt| opt.take()) {
                success_tx.send(Ok(()));
        }
    });

    let error_callback: Closure<dyn FnMut(JsValue)> = 
        brwoser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok()
            .and_then(|mut opt| opt.take()) {
                error_tx.send(Err(anyhow!("Error Loading Image {:#?}", err)));
        }
    });
    
    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);

    complete_rx.await??;

    Ok(iamge)
}

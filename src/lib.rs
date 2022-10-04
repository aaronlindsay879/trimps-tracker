mod data;
mod util;

use data::Data;
use std::time::Duration;
use util::{log, sleep};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sleep(Duration::from_secs(1)).await;

    let mut data = Data::default();
    data.update();

    log!("{data:#?}");
}

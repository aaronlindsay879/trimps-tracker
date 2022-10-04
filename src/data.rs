use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn heliumOnPortal() -> f64;
}

#[derive(Default, Debug)]
pub struct Data {
    helium_on_portal: f64,
}

impl Data {
    pub fn update(&mut self) {
        self.helium_on_portal = heliumOnPortal();
    }
}

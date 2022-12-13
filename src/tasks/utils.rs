use wasm_bindgen::prelude::*;

pub fn str_to_i32(string: &str) -> i32 {
    string.trim().parse().unwrap()
}

#[wasm_bindgen]
extern "C" {
    pub fn log(s: &str);
}

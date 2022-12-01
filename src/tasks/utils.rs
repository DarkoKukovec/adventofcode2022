use wasm_bindgen::prelude::*;

pub fn str_to_i32(string: &str) -> i32 {
    let num: i32 = string.trim().parse().unwrap();
    return num;
}

#[wasm_bindgen]
extern {
    pub fn log(s: &str);
}

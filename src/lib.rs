mod tasks;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exec_test(name: &str) -> String {
    return tasks::test::exec(name);
}

#[wasm_bindgen]
pub fn exec_1(input: &str) -> String {
    return tasks::day_1::exec(input);
}

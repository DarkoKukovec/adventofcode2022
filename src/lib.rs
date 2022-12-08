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

#[wasm_bindgen]
pub fn exec_2(input: &str) -> String {
    return tasks::day_2::exec(input);
}

#[wasm_bindgen]
pub fn exec_3(input: &str) -> String {
    return tasks::day_3::exec(input);
}

#[wasm_bindgen]
pub fn exec_4(input: &str) -> String {
    return tasks::day_4::exec(input);
}

#[wasm_bindgen]
pub fn exec_5(input: &str) -> String {
    return tasks::day_5::exec(input);
}

#[wasm_bindgen]
pub fn exec_6(input: &str) -> String {
    return tasks::day_6::exec(input);
}

#[wasm_bindgen]
pub fn exec_7(input: &str) -> String {
    return tasks::day_7::exec(input);
}

#[wasm_bindgen]
pub fn exec_8(input: &str) -> String {
    return tasks::day_8::exec(input);
}

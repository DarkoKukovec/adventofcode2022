mod tasks;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exec_test(name: &str) -> String {
    tasks::test::exec(name)
}

#[wasm_bindgen]
pub fn exec_1(input: &str) -> String {
    tasks::day_1::exec(input)
}

#[wasm_bindgen]
pub fn exec_2(input: &str) -> String {
    tasks::day_2::exec(input)
}

#[wasm_bindgen]
pub fn exec_3(input: &str) -> String {
    tasks::day_3::exec(input)
}

#[wasm_bindgen]
pub fn exec_4(input: &str) -> String {
    tasks::day_4::exec(input)
}

#[wasm_bindgen]
pub fn exec_5(input: &str) -> String {
    tasks::day_5::exec(input)
}

#[wasm_bindgen]
pub fn exec_6(input: &str) -> String {
    tasks::day_6::exec(input)
}

#[wasm_bindgen]
pub fn exec_7(input: &str) -> String {
    tasks::day_7::exec(input)
}

#[wasm_bindgen]
pub fn exec_8(input: &str) -> String {
    tasks::day_8::exec(input)
}

#[wasm_bindgen]
pub fn exec_9(input: &str) -> String {
    tasks::day_9::exec(input)
}

#[wasm_bindgen]
pub fn exec_10(input: &str) -> String {
    tasks::day_10::exec(input)
}

#[wasm_bindgen]
pub fn exec_11(input: &str) -> String {
    tasks::day_11::exec(input)
}

#[wasm_bindgen]
pub fn exec_12(input: &str) -> String {
    tasks::day_12::exec(input)
}

#[wasm_bindgen]
pub fn exec_13(input: &str) -> String {
    tasks::day_13::exec(input)
}

#[wasm_bindgen]
pub fn exec_14(input: &str) -> String {
    tasks::day_14::exec(input)
}

#[wasm_bindgen]
pub fn exec_15(input: &str) -> String {
    tasks::day_15::exec(input)
}

#[wasm_bindgen]
pub fn exec_16(input: &str) -> String {
    tasks::day_16::exec(input)
}

#[wasm_bindgen]
pub fn exec_17(input: &str) -> String {
    tasks::day_17::exec(input)
}

#[wasm_bindgen]
pub fn exec_18(input: &str) -> String {
    tasks::day_18::exec(input)
}

#[wasm_bindgen]
pub fn exec_19(input: &str) -> String {
    tasks::day_19::exec(input)
}

#[wasm_bindgen]
pub fn exec_20(input: &str) -> String {
    tasks::day_20::exec(input)
}

#[wasm_bindgen]
pub fn exec_21(input: &str) -> String {
    tasks::day_21::exec(input)
}

#[wasm_bindgen]
pub fn exec_22(input: &str) -> String {
    tasks::day_22::exec(input)
}

#[wasm_bindgen]
pub fn exec_23(input: &str) -> String {
    tasks::day_23::exec(input)
}

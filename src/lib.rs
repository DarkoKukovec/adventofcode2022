mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn log(s: &str);
}

fn str_to_i32(string: &str) -> i32 {
    let num: i32 = string.trim().parse().unwrap();
    return num;
}

#[wasm_bindgen]
pub fn exec_test(name: &str) -> String {
    log(&format!("Hello from console, {name}!"));
    return format!("Hello, {name}!");
}

#[wasm_bindgen]
pub fn exec_1(input: &str) -> String {
    let mut calories: Vec<i32> = input
        .split("\n\n")
        .map(|x| x.split("\n").map(str_to_i32))
        .map(|x| x.sum())
        .collect();
    
    calories.sort();
    calories.reverse();

    return format!("{}, {}", calories[0], calories[0..3].iter().sum::<i32>());
}

mod utils;

use std::i32::MAX;

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
    let elves = input.split("\n\n");
    let mut top1 = 0;
    let mut top3 = (0, 0, 0);

    for elv in elves {
        let calories = elv.split("\n").map(|x| str_to_i32(x)).sum();
        if top1 < calories {
            top1 = calories;
        }

        // TODO: This is stupid - refactor
        if top3.0 < calories {
            top3 = (calories, top3.0, top3.1);
        } else if top3.1 < calories {
            top3 = (top3.0, calories, top3.1);
        } else if top3.2 < calories {
            top3 = (top3.0, top3.1, calories);
        }
    }

    return format!("{}, {}", top1, top3.0 + top3.1 + top3.2);
}

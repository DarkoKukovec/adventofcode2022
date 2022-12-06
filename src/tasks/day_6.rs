use std::{collections::HashMap, str::Chars};

fn is_start(seq: Chars, count: usize) -> usize {
    let mut has_match = 0;
    'outer: for (i, _ch) in seq.clone().enumerate() {
        if i < count + 1 {
            continue;
        }
        let mut char_map = HashMap::new();
        for pos in i - count..i {
            let ch = seq.clone().nth(pos).unwrap();
            if char_map.contains_key(&ch) {
                continue 'outer;
            } else {
                char_map.insert(ch, 1);
            }
        }
        has_match = i;
        break;
    }
    has_match
}

pub fn exec(input: &str) -> String {
    let seq = input.chars();

    let start_pos = is_start(seq.clone(), 4);
    let msg_pos = is_start(seq.clone(), 14);

    return format!("{}, {}", start_pos, msg_pos);
}

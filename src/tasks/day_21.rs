use std::collections::HashMap;

enum MonkeyValue {
    Number(i128),
    Operation((String, char, String)),
}

fn get_value(name: &str, map: &HashMap<&str, MonkeyValue>, search_humn: bool) -> Option<i128> {
    if search_humn && name == "humn" {
        return None;
    }
    return match map.get(name).unwrap() {
        MonkeyValue::Number(num) => Some(*num),
        MonkeyValue::Operation((a, op, b)) => {
            let a = get_value(a, map, search_humn);
            let b = get_value(b, map, search_humn);
            if a.is_none() || b.is_none() {
                return None;
            }
            let a = a.unwrap();
            let b = b.unwrap();
            match op {
                '+' => Some(a + b),
                '-' => Some(a - b),
                '*' => Some(a * b),
                '/' => Some(a / b),
                '=' => {
                    if a == b {
                        Some(1)
                    } else {
                        Some(0)
                    }
                }
                _ => panic!("Unknown operation"),
            }
        }
    };
}

fn get_new_a_expected(expected: i128, b: i128, op: char) -> i128 {
    match op {
        '+' => expected - b,
        '-' => expected + b,
        '*' => expected / b,
        '/' => expected * b,
        '=' => b,
        _ => panic!("Unknown operation"),
    }
}

fn get_new_b_expected(expected: i128, a: i128, op: char) -> i128 {
    match op {
        '+' => expected - a,
        '-' => a - expected,
        '*' => expected / a,
        '/' => a / expected,
        '=' => a,
        _ => panic!("Unknown operation"),
    }
}

fn get_humn(name: &str, map: &HashMap<&str, MonkeyValue>, expected: i128) -> Option<i128> {
    if name == "humn" {
        return Some(expected);
    }
    return match map.get(name).unwrap() {
        MonkeyValue::Number(num) => Some(*num),
        MonkeyValue::Operation((a_name, op, b_name)) => {
            let a = get_value(a_name, map, true);
            let b = get_value(b_name, map, true);
            if a.is_none() && b.is_none() {
                panic!("Too many humn found");
            }
            if a.is_some() && b.is_some() {
                panic!("No humn found");
            }
            if a.is_none() {
                get_humn(a_name, map, get_new_a_expected(expected, b.unwrap(), *op))
            } else {
                get_humn(b_name, map, get_new_b_expected(expected, a.unwrap(), *op))
            }
        }
    };
}

pub fn exec(input: &str) -> String {
    let mut input = input
        .lines()
        .map(|x| {
            let mut parts = x.split(": ");
            let name = parts.next().unwrap();
            let value = parts.next().unwrap();
            let num_value = value.parse::<i128>();
            if num_value.is_ok() {
                return (name, MonkeyValue::Number(num_value.unwrap()));
            }
            let mut parts = value.split(" ");
            return (
                name,
                MonkeyValue::Operation((
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().chars().next().unwrap(),
                    parts.next().unwrap().to_string(),
                )),
            );
        })
        .fold(HashMap::new(), |mut h, x| {
            h.insert(x.0, x.1);
            h
        });

    let root_value = get_value("root", &input, false);

    let root = input.get("root").unwrap();
    let new_root = match root {
        MonkeyValue::Number(_) => panic!("Root is a number"),
        MonkeyValue::Operation((a, _op, b)) => {
            MonkeyValue::Operation((a.to_string(), '=', b.to_string()))
        }
    };
    input.insert("root", new_root);
    let humn_value = get_humn("root", &input, 1).unwrap();

    return format!("{}, {}", root_value.unwrap(), humn_value);
}

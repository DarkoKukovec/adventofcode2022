use std::collections::HashMap;

enum MonkeyValue {
    Number(i128),
    Operation((String, char, String)),
}

fn get_value(name: &str, map: &HashMap<&str, MonkeyValue>) -> i128 {
    return match map.get(name).unwrap() {
        MonkeyValue::Number(num) => *num,
        MonkeyValue::Operation((a, op, b)) => {
            let a = get_value(a, map);
            let b = get_value(b, map);
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                '=' => {
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Unknown operation"),
            }
        }
    };
}

pub fn exec(input: &str) -> String {
    let input = input
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

    let root_value = get_value("root", &input);

    let humn_value = 0;

    return format!("{}, {}", root_value, humn_value);
}

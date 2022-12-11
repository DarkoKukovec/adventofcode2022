use std::collections::HashMap;

#[derive(Clone)]
struct Monkey {
    num: i32,
    items: Vec<u128>,
    op: Vec<String>,
    test: u128,
    on_true: i32,
    on_false: i32,
    inspections: u128,
}

fn calculate_worry(item: u128, op: &Vec<String>, div: u128, modulo: u128) -> u128 {
    let mut worry = item;
    let a = op[0].parse::<u128>().unwrap_or(worry);
    let b = op[2].parse::<u128>().unwrap_or(worry);
    match op[1].as_str() {
        "+" => worry = a + b,
        _ => worry = a * b,
    }

    return (worry as f64 / div as f64).floor() as u128 % modulo;
}

fn runner(mut monkeys: HashMap<i32, Monkey>, order: &Vec<i32>, rounds: i32, div: u128) -> u128 {
    let modulo: u128 = monkeys.values().map(|m| m.test).product();
    for _round in 0..rounds {
        for monkey_id in order.clone() {
            loop {
                let mut monkey = monkeys.get_mut(&monkey_id).unwrap();
                if monkey.items.len() == 0 {
                    break;
                }
                monkey.inspections += 1;
                let item = monkey.items.remove(0);
                let worry = calculate_worry(item, &monkey.op, div, modulo);
                let on_true = monkey.on_true;
                let on_false = monkey.on_false;
                if worry % monkey.test == 0 {
                    let next_monkey = monkeys.get_mut(&on_true).unwrap();
                    next_monkey.items.push(worry);
                } else {
                    let next_monkey = monkeys.get_mut(&on_false).unwrap();
                    next_monkey.items.push(worry);
                }
            }
        }
    }

    let mut res: Vec<&Monkey> = monkeys.values().into_iter().collect();
    res.sort_by_key(|x| x.inspections);
    res.reverse();
    res[0].inspections * res[1].inspections
}

pub fn exec(input: &str) -> String {
    let input = input
        .split("\n\n")
        .map(|monkey| {
            let mut data = monkey.split("\n").map(|x| x.trim());
            let num = data
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .split(":")
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let items = data
                .next()
                .unwrap()
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split(", ")
                .map(|x| x.trim().parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            let op = data
                .next()
                .unwrap()
                .split("=")
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let test = data
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<u128>()
                .unwrap();
            let on_true = data
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let on_false = data
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            return Monkey {
                num,
                items,
                op,
                test,
                on_true,
                on_false,
                inspections: 0,
            };
        })
        .collect::<Vec<Monkey>>();

    let mut monkeys = HashMap::<i32, Monkey>::new();
    let mut order: Vec<i32> = Vec::new();
    for monkey in input {
        let num = monkey.num;
        monkeys.insert(num, monkey);
        order.push(num);
    }

    let res1 = runner(monkeys.clone(), &order, 20, 3);
    let res2 = runner(monkeys.clone(), &order, 10000, 1);

    return format!("{}, {}", res1, res2);
}

use std::cmp::Ordering;

#[derive(Debug)]
enum Type {
    Num(i32),
    List(Vec<Type>),
}

#[derive(Debug, PartialEq)]
enum CompareResult {
    RightOrder,
    WrongOrder,
    Undetermined,
}

fn parse_list(list: &str) -> Type {
    let mut chars = list[1..list.len() - 1].chars();
    let mut result = Vec::new();

    let mut num: i32 = 0;
    let mut num_set = false;
    let mut list = String::new();
    let mut list_depth = 0;
    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                if list_depth > 0 {
                    list.push(c);
                } else {
                    num = num * 10 + c.to_digit(10).unwrap() as i32;
                    num_set = true;
                }
            }
            '[' => {
                list_depth += 1;
                list.push(c);
            }
            ']' => {
                list_depth -= 1;
                list.push(c);
                if list_depth == 0 {
                    result.push(parse_list(&list));
                    list.clear();
                }
            }
            ',' => {
                if list_depth > 0 {
                    list.push(c);
                } else if num_set {
                    result.push(Type::Num(num));
                    num = 0;
                    num_set = false;
                }
            }
            _ => (),
        }
    }
    if num_set {
        result.push(Type::Num(num));
    }
    return Type::List(result);
}

fn compare(left: &Vec<Type>, right: &Vec<Type>) -> CompareResult {
    let mut left = left.iter().collect::<Vec<&Type>>();
    let mut right = right.iter().collect::<Vec<&Type>>();
    left.reverse();
    right.reverse();
    loop {
        let left_item = left.pop();
        let right_item = right.pop();
        let res = match (left_item, right_item) {
            (Some(Type::Num(l)), Some(Type::Num(r))) => {
                let mut res = CompareResult::Undetermined;
                if l < r {
                    res = CompareResult::RightOrder;
                } else if l > r {
                    res = CompareResult::WrongOrder;
                }
                res
            }
            (Some(Type::List(l)), Some(Type::List(r))) => compare(&l, &r),
            (Some(Type::Num(l)), Some(Type::List(r))) => compare(&vec![Type::Num(l.clone())], &r),
            (Some(Type::List(l)), Some(Type::Num(r))) => compare(&l, &vec![Type::Num(r.clone())]),
            (None, None) => break,
            (None, _) => CompareResult::RightOrder,
            (_, None) => CompareResult::WrongOrder,
        };
        if res != CompareResult::Undetermined {
            return res;
        }
    }
    CompareResult::Undetermined
}

pub fn exec(input: &str) -> String {
    let pairs = input
        .split("\n\n")
        .map(|x| {
            let mut pair = x.split("\n");
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .map(|pair| (parse_list(pair.0), parse_list(pair.1)));

    // Part 1
    let comparison_results = pairs
        .clone()
        .map(|pair| {
            if let Type::List(left) = pair.0 {
                if let Type::List(right) = pair.1 {
                    return compare(&left, &right);
                }
            }
            CompareResult::Undetermined
        })
        .collect::<Vec<CompareResult>>();

    let mut sum_indexes = 0;
    for pair_index in 0..comparison_results.len() {
        if comparison_results[pair_index] == CompareResult::RightOrder {
            sum_indexes += pair_index + 1;
        }
    }

    // Part 2
    let divider_1 = Type::List(vec![Type::List(vec![Type::Num(2)])]);
    let divider_2 = Type::List(vec![Type::List(vec![Type::Num(6)])]);
    let div_1_str = format!("{:?}", divider_1);
    let div_2_str = format!("{:?}", divider_2);
    let mut list = pairs.map(|x| [x.0, x.1]).flatten().collect::<Vec<Type>>();
    list.push(divider_1);
    list.push(divider_2);

    list.sort_by(|a, b| {
        if let Type::List(left) = a {
            if let Type::List(right) = b {
                let cmp = compare(left, right);
                match cmp {
                    CompareResult::RightOrder => return Ordering::Less,
                    CompareResult::WrongOrder => return Ordering::Greater,
                    CompareResult::Undetermined => return Ordering::Equal,
                }
            }
        }
        return Ordering::Equal;
    });

    let mut mul_div_indexes = 1;
    for pair_index in 0..list.len() {
        let item = format!("{:?}", list[pair_index]);
        if item == div_1_str || item == div_2_str {
            mul_div_indexes *= pair_index + 1;
        }
    }

    return format!("{}, {}", sum_indexes, mul_div_indexes);
}

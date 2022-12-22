fn mix(numbers: &Vec<(usize, i128)>, times: usize) -> i128 {
    let initial = numbers.clone();
    let size = numbers.len() as i128;
    let mut numbers = numbers.clone();

    for _i in 0..times {
        for item in &initial {
            let current_pos = numbers
                .clone()
                .iter()
                .enumerate()
                .find(|x| x.1 == item)
                .unwrap()
                .0;
            let mut new_pos = current_pos as i128 + item.1;
            new_pos %= size - 1;
            if new_pos < 0 {
                new_pos += size - 1;
            }
            numbers.remove(current_pos);
            numbers.insert(new_pos as usize, *item);
        }
    }

    let zero = numbers.iter().enumerate().find(|x| x.1 .1 == 0).unwrap().0;
    numbers[(zero + 1000) % size as usize].1
        + numbers[(zero + 2000) % size as usize].1
        + numbers[(zero + 3000) % size as usize].1
}

pub fn exec(input: &str) -> String {
    let numbers = input
        .lines()
        .enumerate()
        .map(|(i, x)| (i, x.parse::<i128>().unwrap()))
        .collect::<Vec<_>>();

    let encoded = numbers
        .iter()
        .map(|(i, x)| (*i, x * 811589153))
        .collect::<Vec<_>>();

    return format!("{}, {}", mix(&numbers, 1), mix(&encoded, 10));
}

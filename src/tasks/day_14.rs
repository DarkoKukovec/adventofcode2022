use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum SpotType {
    Rock,
    Sand,
    Empty,
}

pub fn exec(input: &str) -> String {
    let map = input
        .split("\n")
        .map(|path| {
            path.split(" -> ")
                .map(|point| {
                    let mut point = point.split(",").map(|x| x.parse::<i32>().unwrap());
                    (point.next().unwrap(), point.next().unwrap())
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .fold(HashMap::new(), |map, path| {
            let mut map = map;
            for i in 1..path.len() {
                let start_x = *[path[i - 1].0, path[i].0].iter().min().unwrap();
                let start_y = *[path[i - 1].1, path[i].1].iter().min().unwrap();
                let end_x = *[path[i - 1].0, path[i].0].iter().max().unwrap();
                let end_y = *[path[i - 1].1, path[i].1].iter().max().unwrap();
                if start_x == end_x {
                    for y in start_y..=end_y {
                        map.entry(y)
                            .or_insert(HashMap::new())
                            .entry(start_x)
                            .or_insert(SpotType::Rock);
                    }
                } else if start_y == end_y {
                    for x in start_x..=end_x {
                        map.entry(start_y)
                            .or_insert(HashMap::new())
                            .entry(x)
                            .or_insert(SpotType::Rock);
                    }
                }
            }
            map
        });

    let max_depth = **&map.keys().max().unwrap();
    let empty: HashMap<i32, SpotType> = HashMap::new();

    // Part 1
    let mut map_1 = map.clone();
    let mut sand_count_1 = 0;
    'outer: loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        loop {
            if sand_y >= max_depth {
                break 'outer;
            }
            let next_row = map_1.get(&(sand_y + 1)).unwrap_or(&empty);
            let current_pos = next_row.get(&sand_x).unwrap_or(&SpotType::Empty);
            let left_pos = next_row.get(&(sand_x - 1)).unwrap_or(&SpotType::Empty);
            let right_pos = next_row.get(&(sand_x + 1)).unwrap_or(&SpotType::Empty);
            if current_pos == &SpotType::Empty {
                sand_y += 1;
            } else if left_pos == &SpotType::Empty {
                sand_y += 1;
                sand_x -= 1;
            } else if right_pos == &SpotType::Empty {
                sand_y += 1;
                sand_x += 1;
            } else {
                break;
            }
        }
        sand_count_1 += 1;
        map_1
            .entry(sand_y)
            .or_insert(HashMap::new())
            .entry(sand_x)
            .or_insert(SpotType::Sand);
    }

    // Part 2
    let mut map_2 = map.clone();
    let mut sand_count_2 = 0;
    'outer: loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        loop {
            if sand_y == max_depth + 1 {
                break;
            }
            let next_row = map_2.get(&(sand_y + 1)).unwrap_or(&empty);
            let current_pos = next_row.get(&sand_x).unwrap_or(&SpotType::Empty);
            let left_pos = next_row.get(&(sand_x - 1)).unwrap_or(&SpotType::Empty);
            let right_pos = next_row.get(&(sand_x + 1)).unwrap_or(&SpotType::Empty);
            if current_pos == &SpotType::Empty {
                sand_y += 1;
            } else if left_pos == &SpotType::Empty {
                sand_y += 1;
                sand_x -= 1;
            } else if right_pos == &SpotType::Empty {
                sand_y += 1;
                sand_x += 1;
            } else {
                break;
            }
        }
        sand_count_2 += 1;

        if sand_x == 500 && sand_y == 0 {
            break 'outer;
        }
        map_2
            .entry(sand_y)
            .or_insert(HashMap::new())
            .entry(sand_x)
            .or_insert(SpotType::Sand);
    }

    return format!("{}, {}", sand_count_1, sand_count_2);
}

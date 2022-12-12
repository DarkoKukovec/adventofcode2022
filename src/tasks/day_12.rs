pub fn exec(input: &str) -> String {
    let mut x = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map: Vec<Vec<i32>> = input
        .split("\n")
        .map(|l| {
            let mut y = 0;
            let line = l
                .chars()
                .map(|c| {
                    y += 1;
                    match c {
                        'S' => {
                            start = (x, y - 1);
                            'a' as i32 - 96
                        }
                        'E' => {
                            end = (x, y - 1);
                            'z' as i32 - 96
                        }
                        _ => c as i32 - 96,
                    }
                })
                .collect::<Vec<i32>>();
            x += 1;
            return line;
        })
        .collect();

    let mut paths: Vec<Vec<(usize, usize)>> = vec![vec![end]];
    let mut used = vec![end];
    let mut new_paths: Vec<Vec<(usize, usize)>> = vec![];
    let mut shortest_path: usize = 0;
    let mut shortest_to_a: usize = 0;

    'outer: loop {
        for path in &paths {
            let last = path.last().unwrap();
            if (map[last.0][last.1] + 96) as u8 as char == 'a' && shortest_to_a == 0 {
                shortest_to_a = path.len() - 1;
            }
            if last.0 == start.0 && last.1 == start.1 {
                break 'outer;
            }
            let mut possible: Vec<(usize, usize)> = vec![];
            if last.0 > 0 {
                possible.push((last.0 - 1, last.1));
            }
            if last.0 < map.len() - 1 {
                possible.push((last.0 + 1, last.1));
            }
            if last.1 > 0 {
                possible.push((last.0, last.1 - 1));
            }
            if last.1 < map[0].len() - 1 {
                possible.push((last.0, last.1 + 1));
            }

            for next in possible {
                if map[next.0][next.1] - map[last.0][last.1] >= -1 {
                    if !used.contains(&next) {
                        let mut new_path: Vec<(usize, usize)> = path.clone();
                        new_path.push(next);
                        new_paths.push(new_path);
                        used.push(next);
                    }
                }
            }
        }
        shortest_path += 1;
        paths.clear();
        paths.append(&mut new_paths);
    }

    return format!("{}, {}", shortest_path, shortest_to_a);
}
